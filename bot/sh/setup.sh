rsync -avh /daniel/ENV/BlackcoinDiscordBot admin@vps.blackcoin.nl:/home/admin/legacyBot/

# ssh blkVps << EOF
# docker compose -f /home/daniel/ENV/BlackcoinDiscordBot/docker-compose.yaml up -d
# EOF