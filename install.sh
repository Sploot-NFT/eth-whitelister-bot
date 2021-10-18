sudo mkdir /usr/local/bin/eth-whitelister

echo \{\} > /usr/local/bin/eth-whitelister/whitelist.json

echo Input Discord Bot token:
read -r token

echo Input Discord Application ID:
read -r application_id

echo Input admin role ID:
read -r admin_role

echo Input admin server:
read -r admin_server

echo Input deadline as Unix timestamp \(can be changed later\)
read -r deadline

sudo echo "{
  \"open\": true,
  \"admin_role\": \"$admin_role\",
  \"admin_server\": \"$admin_server\",
  \"application_id\": \"$application_id\",
  \"token\": \"$token\",
  \"deadline\": $deadline}" > /usr/local/bin/eth-whitelister/config.json

sudo curl -L -o /etc/systemd/system/eth-whitelister.service https://github.com/Sploot-NFT/eth-whitelister-bot/releases/download/latest/eth-whitelister.service
sudo curl -L -o /usr/local/bin/eth-whitelister/eth-whitelister https://github.com/Sploot-NFT/eth-whitelister-bot/releases/download/latest/eth_whitelister
sudo chmod +x /usr/local/bin/eth-whitelister/eth-whitelister

sudo systemctl enable eth-whitelister
sudo systemctl start eth-whitelister