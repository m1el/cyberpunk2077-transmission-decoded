# Cyberpunk 2077 stream, decoded

On 2018-08-27, CD Projekt RED streamed base64-encoded text on their twitch channel.

This repo contains code used to decode the stream.

# The entire process

The file `pipeline.sh` contains commands used to decode the stream.

```bash
# 1) Downloading the stream using youtube-dl:
youtube-dl.exe https://www.twitch.tv/videos/302423092 -o full_stream.mp4`

# 2) Converting the stream to images:
ffmpeg -i full_stream.mp4 -vf fps=1/10 shots/%04d.png

# 3) Building the dumb OCR program:
cargo build --release --manifest-path dec/Cargo.toml

# 4) Running OCR on images:
dec/target/release/dec > cyberpunk2077-raw.b64

# 5) Running deduplication/error correction:
ConsoleApplication69/ConsoleApplication69/bin/Debug/ConsoleApplication69.exe

# 6) Finally, decoding base64:
base64 -id < cyberpunk2077-decoded.png.b64 > cyberpunk2077-decoded.png
```

# OCR Process (step 4)

The source code is in `dec/src/main.rs`

The process used for OCR is very dumb:

1. Hand-craft an alphabet of images, see `alphabet.png`
2. Obtain characters from the source image using a hardcoded grid (lines 37..40)
3. Find which character from the alphabet has the smallest difference with current character (lines 51..61)

# Deduplication

The source code is in `ConsoleApplication69/ConsoleApplication69/Program.cs`

1. Throw away duplicate files
2. Mark rows that appear at least twice as "legit"
3. Start outputting rows
4. If encountered end of current file, or "illegit" row,
   find a copy the last "legit row", which is followed by a "legit" row,
   goto step 3.
5. Hacks
