use std::fs;
use std::io::Write;
use std::collections::HashMap;
use web3;
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandOptionType,
                ApplicationCommandInteraction,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};

struct Handler;

fn timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    return since_the_epoch;
}

async fn check_valid(account:&str) -> bool {
    let account = account.parse::<web3::types::Address>();

    let valid = match account {
        Ok(_) => true,
        Err(_) => false
    };

    return valid;
}

async fn resolve_ens(ens_name: &str) -> Result<String, &str> {
    let query = format!("{{
  domains(where: {{name:\"{}\"}}) {{
    resolvedAddress {{
      id
    }}
  }}
}}", ens_name);
    let mut map = HashMap::new();
    map.insert("query", query);
    let url = "https://api.thegraph.com/subgraphs/name/ensdomains/ens";

    let client = reqwest::Client::new();
    let res = client.post(url)
        .json(&map)
        .send()
        .await;
    let json:serde_json::Value = res.unwrap().json().await.unwrap();
    let text = &json["data"]["domains"][0]["resolvedAddress"]["id"].as_str();
    let text = match text {
        Some(x) => Ok(x.to_string()),
        None => Err("Unable to resolve ENS address")
    };

    return text;
}

async fn update_whitelist(user_id: &str, address: &str) -> bool { // Returns bool if entry updated rather than created
    let old = fs::read_to_string("whitelist.json").expect("Couldn't read whitelist.json");
    let mut json: HashMap<String, serde_json::Value> = serde_json::from_str(&old)
    .expect("whitelist.json is not proper JSON");

    let exists = match json.get(user_id) {
        Some(_) => true,
        None => false
    };

    json.insert(user_id.to_string(), serde_json::Value::from(address.to_string()));
    let out = serde_json::to_string_pretty(&json).expect("unable to serialize JSON");
    let mut file = fs::File::create("whitelist.json").expect("Failed to open whitelist.json for writing");
    file.write_all(out.as_bytes()).expect("Failed to write to whitelist.json");

    return exists;
}

pub struct DeadlineStruct;

impl serenity::prelude::TypeMapKey for DeadlineStruct {
    type Value = u64;
}

async fn whitelist(command: &ApplicationCommandInteraction, ctx: &Context) -> String {
    let data = &ctx.data.read().await;
    let deadline:&u64 = data.get::<DeadlineStruct>().unwrap();
    if &timestamp() > deadline {
        return "You missed the deadline, sorry!".to_string();
    }

    let mut address = command.data.options.get(0).expect("Could not get first option")
        .value.as_ref().expect("Could not reference value")
        .as_str().unwrap().to_string();

    if address.contains(".") {
        let resolved = resolve_ens(&address).await;
        match resolved {
            Ok(x) => address = x,
            Err(x) => return format!("Error encountered: `{}`", x.to_string())
        };
    }

    if check_valid(&address).await != true{
        return "Error encountered: `Invalid Eth address`".to_string();
    };

    let updated = update_whitelist(&command.user.id.to_string(), &address).await;

    return match updated {
        false => format!("You are now registered as `{}`", address),
        true => format!("Registration updated to `{}`", address)
    };
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command.name("whitelist").description("Whitelist your Eth address!").create_option(|option| {
                        option
                            .name("address")
                            .description("Your Eth address")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
                })
        })
        .await.expect("Failed to register slash commands");

    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                "whitelist" => {
                    whitelist(&command, &ctx).await
                }
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let config = fs::read_to_string("config.json").expect("Couldn't read config.json");  // Read config file in
    let config: HashMap<String, serde_json::Value> = serde_json::from_str(&config)  // Convert config string into HashMap
    .expect("config.json is not proper JSON");

    let token = config.get("token").unwrap().as_str().unwrap();
    let application_id: u64 = config.get("application_id").unwrap().as_str().unwrap()
        .trim()
        .parse()
        .expect("application id is not a valid id");
    let deadline: u64 = config.get("deadline").unwrap().as_str().expect("Deadline not found in config").parse().expect("Couldn't convert deadline to integer");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<DeadlineStruct>(deadline);
    }

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
