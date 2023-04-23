rsync -avh --exclude-from='.rsync-exclude' /d/ENV/wtf admin@vps.blackcoin.nl:/home/admin/ENV

ssh blkVps << EOF
docker compose -f /home/admin/ENV/wtf/docker-compose.yaml down
docker compose -f /home/admin/ENV/wtf/docker-compose.yaml up -d
EOF