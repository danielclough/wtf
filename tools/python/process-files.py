import whisper
import os

model = whisper.load_model("small")

# transcribe each file in dir
dir = 'tools/final/'
for filename in os.listdir(dir):

    # load audio and pad/trim it to fit 30 seconds
    audio = whisper.load_audio(dir + filename)

    # make log-Mel spectrogram and move to the same device as the model
    mel = whisper.log_mel_spectrogram(audio).to(model.device)

    # detect the spoken language
    _, probs = model.detect_language(mel)
    print(f"Detected language: {max(probs, key=probs.get)}")

    # decode the audio
    options = whisper.DecodingOptions()
    result = whisper.decode(model, mel, options)

    # print the recognized text
    print(result.text)