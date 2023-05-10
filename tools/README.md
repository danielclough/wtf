# Tools 

## Noise Gate

Files must be WAV.

ffmpeg -i song.mp3 -acodec pcm_u8 -ar 22050 song.wav



```console
$ cargo run --release --example wav-splitter -- \
    --output-dir output \
    --threshold 50 \
    --release-time 0.3 \
    /input/EHTX2-Apr-27-2023-1230Z.wav

$ ls output
```

Use with (Whisper)[https://github.com/openai/whisper]:

`pip install git+https://github.com/openai/whisper.git`
To update the package to the latest version of this repository, please run:
`pip install --upgrade --no-deps --force-reinstall git+https://github.com/openai/whisper.git`


```bash
whisper_in='data/tornado-warning-ground.wav'

~/.local/bin/whisper $whisper_in --language English --output_dir whisper_out --output_format json --model medium
```

## Stable Diffusion

"Digital painting fantasy artstation god rays Alan Bean Alex Grey Alex Ross Anne Stokes Beeple Eduardo Kobra Hayao Miyazaki Hideaki Anno "  -n4 -C10


## R&C

| | politics | metaphysics | harm principle | legal positivism |
| -- | -- | -- | -- | -- |
| politics | -- | -- | -- | -- |
| metaphysics |-- | -- | -- | -- |
| harm principle |-- | -- | -- | -- |
| legal positivism |-- | -- | -- | -- |


## Audacity

low cut - 250 starts to effect voice

high cut taper off around 2k

compressor

normalization

noise reduction

reverb


## Cat Vids

ffmpeg -i clipped.MTS -vcodec copy -vbsf h264_mp4toannexb -acodec copy part1.ts
ffmpeg -i 00011.MTS -vcodec copy -vbsf h264_mp4toannexb -acodec copy part2.ts
ffmpeg -i 00012.MTS -vcodec copy -vbsf h264_mp4toannexb -acodec copy part3.ts
cat part1.ts part2.ts part3.ts > parts.ts
ffmpeg -y -i parts.ts -acodec copy -ar 44100 -ab 96k -coder ac -vbsf h264_mp4toannexb parts.mkv