#!/bin/bash

input=$1

file_name=`echo $input | cut -d '.' -f 1 | rev | cut -d'/' -f1 | rev`
not_wav=`echo $input | cut -d '.' -f 2 | grep -v wav`

if [ -n $not_wav ]; then
    wav_file=`echo "wav_output/${file_name}.wav"`

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
    --threshold 15 \
    --release-time 0.3 \
    $wav_file