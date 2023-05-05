#!/bin/bash

input=$1

file_name=`echo $input | cut -d '.' -f 1 | rev | cut -d'/' -f1 | rev`
not_wav=`echo $input | cut -d '.' -f 2 | grep -v wav`

if [ -n $not_wav ]; then
    wav_file=`echo "wav_files/${file_name}.wav"`

    echo -e "
$input outup to: 
    $wav_file
"

    ffmpeg -i $input -acodec pcm_u8 -ar 22050 $wav_file
else
    wav_file=$input
fi

cargo run --release --example wav-splitter -- \
    --output-dir output \
    --threshold 5 \
    --release-time .3 \
    $wav_file

files=`ls output`

for file in ${files[@]}; do
    echo "file 'output/$file'" >> file_list.txt
done

ffmpeg -f concat -i file_list.txt -c copy final/final.wav
ffmpeg -i final/final.wav -c:a libopus -b:a 96K final/final.opus

rm file_list.txt
rm output/*