#!/bin/bash

# Windows
###############################################
# Install mingw64 from https://sourceforge.net/projects/mingw-w64/ and pick x86_64 posix-seh [save in /C/mingw-w64]
# Install mingw64 from http://msys2.github.io/ [save in /C/msys64]
# pacman -Sy pacman-mirrors
# pacman -S git make diffutils tar mingw-w64-x86_64-python mingw-w64-x86_64-cmake mingw-w64-x86_64-gcc mingw-w64-x86_64-ninja
# pacman -S mingw-w64-x86_64-make mingw-w64-x86_64-clang mingw-w64-x86_64-llvm zip
# add to path, eg: 
# PATH="/C/mingw-w64/x86_64-8.1.0-posix-seh-rt_v6-rev0/mingw64/bin:/c/Users/framp/.cargo/bin:/C/msys64/mingw64/bin:/C/msys64/usr/bin"
# Install rust-init with x86_64-pc-windows-gnu target

if [[ -z $2 ]]; then
    echo "Usage: $0 PLATFORM[win|mac] VERSION[x.y.z]"
    exit 1
fi
if [ $1 == "win" ]; then
    cargo build --release --target x86_64-pc-windows-gnu
    TARGET=x86_64-pc-windows-gnu
elif [ $1 == "mac"]; then
    cargo build --release
else 
    echo "Usage: $0 PLATFORM[win|mac] VERSION[x.y.z]"
    exit 1
fi
rm -rf joystick-mapper*.zip
mkdir -p build
cp target/$TARGET/release/joystick-mapper build
cp examples/sample.conf build/joystick-mapper.conf
echo "Source code and instructions here: https://github.com/framp/joystick-mapper" > build/README.txt
cd build
zip joystick-mapper-$1-$2.zip * 
cd ..
mv build/*.zip . 
rm build/joystick-mapper
cp target/$TARGET/release/joystick-mapper-among-us build
cp examples/among-us.conf build/joystick-mapper.conf
cd build
zip joystick-mapper-among-us-$1-$2.zip * 
cd ..
mv build/*.zip .
rm -rf build
