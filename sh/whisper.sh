rsync -e 'ssh -p 44322' --exclude-from='.rsync-exclude' -avh /home/daniel/git/wtf/tools/final/ daniel@vps.blackcoin.nl:/home/daniel/whisper

ssh -p 44322 daniel@vps.blackcoin.nl << EOF

    for whisper_in in /home/daniel/whisper/*.opus; do
        /home/daniel/.local/bin/whisper $whisper_in --language English --output_dir whisper_out --output_format json --model medium
        rm $whisper_in
        echo "output/$whisper_in"
    done;
EOF