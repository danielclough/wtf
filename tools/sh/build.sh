backup=/home/daniel/git/wtf/
server=daniel@vps.blackcoin.nl
server_dir=/home/daniel/WTF

rsync -e 'ssh -p 44322' --exclude-from='.rsync-exclude' -avh "$backup" "${server}:${server_dir}"

ssh -p 44322 ${server} << EOF
    docker compose -f ${server_dir}/docker-compose-db.yaml up -d --build
    docker compose -f ${server_dir}/docker-compose.yaml up -d --build
EOF