#!/bin/sh
echo -e "\n[profile.release]\nopt-level = 0" >> Cargo.toml
cargo build --release 
git checkout Cargo.toml
rm -rf joystick-mapper*.zip
mkdir -p build
cp target/release/joystick-mapper build
cp examples/sample.conf build/joystick-mapper.conf
echo "Source code and instructions here: https://github.com/framp/joystick-mapper" > build/README.txt
cd build
zip joystick-mapper-mac-$1.zip * 
cd ..
mv build/*.zip . 
rm build/joystick-mapper
cp target/release/joystick-mapper-among-us build
cp examples/among-us.conf build/joystick-mapper.conf
cd build
zip joystick-mapper-among-us-mac-$1.zip * 
cd ..
mv build/*.zip .
rm -rf build