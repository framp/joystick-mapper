#!/bin/sh
export LIBCLANG_PATH="/C/Program Files/LLVM/bin/libclang.dll"
export PATH="/C/Program Files/LLVM/bin/:/C/tools/opencv-build/install/x86/bin/:$PATH"
export OPENCV_INCLUDE_PATHS="/C/tools/opencv/build/include"
export OPENCV_LINK_LIBS="libopencv_core412,libopencv_imgproc412"
export OPENCV_LINK_PATHS="/C/tools/opencv-build/install/x86/mingw/lib"
cargo build --release
rm joystick-mapper-win*.zip
mkdir build
cp target/release/joystick-mapper.exe build
cp examples/sample.conf build/joystick-mapper.conf
echo "Source code and instructions here: https://github.com/framp/joystick-mapper" > build/README.txt
cd build
/C/Program\ Files/7-Zip/7z.exe a joystick-mapper-win-$1.zip *
cd ..
mv build/*.zip . 
rm build/joystick-mapper.exe
cp target/release/joystick-mapper-among-us.exe build
cp examples/among-us.conf build/joystick-mapper.conf
cp /C/tools/opencv/build/x64/mingw/bin/libopencv_core412.dll build
cp /C/tools/opencv/build/x64/mingw/bin/libopencv_imgproc412.dll build
cd build
/C/Program\ Files/7-Zip/7z.exe a joystick-mapper-among-us-win-$1.zip *
cd ..
mv build/*.zip .
rm -rf build
