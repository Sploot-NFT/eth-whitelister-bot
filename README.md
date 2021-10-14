# Ethereum Whitelister Bot
Allow Discord users to whitelist one Eth address in a JSON file.

## Setup
Create a file called `config.json` next to the executable. Its contents should be as follows:  
`{"token": "DISCORD_TOKEN", "application_id": "DISCORD_APPLICATION_ID", "deadline": "UNIX_TIMESTAMP, beyond which registrations are closed"}`  
Also create a file called `whitelist.json` with contents `{}`

## Usage
Just run the executable provided (Linux only unless you build from source). Then add the bot to your server. It will respond to `/whitelist [address]` and `/ping`.  
It will resolve ENS domains and validate the addresses, then store them in `whitelist.json` attached to the User ID.
