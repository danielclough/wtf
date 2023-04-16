rsync -avh /daniel/ENV/BlackcoinDiscordBot daniel@vps.blackcoin.nl:/home/daniel/ENV

ssh blkVps << EOF
docker compose -f /home/daniel/ENV/BlackcoinDiscordBot/docker-compose.yaml down
docker compose -f /home/daniel/ENV/BlackcoinDiscordBot/docker-compose.yaml up -d
EOF
# sudo cp /home/daniel/ENV/BlackcoinDiscordBot/config.production.json /home/daniel/ENV/BlackcoinDiscordBot/config.production.json 