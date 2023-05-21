backup=/home/daniel/git/wtf/tools/final/
server=daniel@vps.blackcoin.nl
server_dir=/home/daniel/WTF/tools/final

rsync -e 'ssh -p 44322 -i /home/daniel/.ssh/blk_ed25519' --exclude-from='.rsync-exclude' -avh "$backup" "${server}:${server_dir}"

ssh -p 44322 -i /home/daniel/.ssh/blk_ed25519 daniel@vps.blackcoin.nl << EOF
    cd  ${server_dir}
    /home/daniel/.local/bin/whisper final.ogg --language English --output_dir whisper_out --output_format json --model base
    rm final.ogg
EOF