
# this is how the pipile works, roughly
set -e
echo "you shouldn't run this file, it's not meant to be ran" && exit 1
youtube-dl.exe https://www.twitch.tv/videos/302423092 -o full_stream.mp4
ffmpeg -i full_stream.mp4 -vf fps=1/10 shots/%04d.png
cargo build --release --manifest-path dec/Cargo.toml
dec/target/release/dec > cyberpunk2077-raw.b64
# you need to compile the C# thingy
ConsoleApplication69/ConsoleApplication69/bin/Debug/ConsoleApplication69.exe
base64 -id < cyberpunk2077-decoded.png.b64 > cyberpunk2077-decoded.png
