#!/bin/sh
# install chocolatey
choco install llvm opencv
export LIBCLANG_PATH="/C/Program Files/LLVM/bin/libclang.dll"
export PATH="/C/Program Files/LLVM/bin/:/C/tools/opencv/build/x64/vc15/bin:$PATH"
export OPENCV_INCLUDE_PATHS="/C/tools/opencv/build/include"
export OPENCV_LINK_LIBS="opencv_world412"
export OPENCV_LINK_PATHS="C:/tools/opencv/build/x64/vc15/lib"
cargo build --release
