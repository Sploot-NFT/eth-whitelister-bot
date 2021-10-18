# Ethereum Whitelister Bot
Allow Discord users to whitelist one Eth address in a JSON file.

## Setup
`curl -o install.sh https://github.com/Sploot-NFT/eth-whitelister-bot/releases/download/latest/install.sh`  
`chmod +x install.sh`  
`sudo ./install.sh`  

The installer will ask you to input the details it needs and will be installed as a system service.

## Usage
### Whitelist
`/whitelist murrax2.eth`  
Whitelists your Eth address.  

### Ping
`/ping`  
Verify the bot is connected.

### Open (Admin role only)
`/open`
Open submissions

### Close (Admin role only)
`/close`  
Close submissions  

### Deadline (Admin role only)
`/deadline 1634566307`  
Sets the deadline in Unix time (submissions are automatically closed after this point)  

### Export (Admin role only)
`/export`  
Uploads the whitelist to Discord  

### Clear (Admin role only)
`/clear confirm`
Clears the whitelist, but creates a backup in `/usr/local/bin/eth-whitelister`  