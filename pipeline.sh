
# this is how the pipile works, roughly
set -e
ffmpeg -i full_stream.mp4 -vf fps=1/10 shots/%04d.png
cd dec
cargo build --release
cd ..
dec/target/release/dec > cyberpunk2077-raw.b64
# you need to compile the C# thingy
ConsoleApplication69/ConsoleApplication69/bin/Debug/ConsoleApplication69.exe
base64 -id < cyberpunk2077-decoded.png.b64 > cyberpunk2077-decoded.png
