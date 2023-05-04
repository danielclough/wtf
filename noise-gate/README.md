# Noise Gate

## Getting Started

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