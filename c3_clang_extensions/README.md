# C3

Internal for the C3 project.

## Building

 * Build [LLVM 4 and Clang](http://releases.llvm.org/download.html) from source (`libclang.a` is needed, and pre-built packages won't have it).
 * Add directory containing `llvm-config` to your `PATH`, or set `LLVM_CONFIG_PATH` env variable poiting to the `llvm-config` executable file.
 * Set `LIBCLANG_INCLUDE_PATH` pointing to Clang's include directory (`<clang install dir>/clang/include/`)

### Building clang

This will require 20GB of disk space and 16GB of RAM.

 * Install cmake, subversion, build-essential, libffi-dev, libncursesw5-dev
 * `curl -LO http://releases.llvm.org/4.0.1/llvm-4.0.1.src.tar.xz`
 * `tar xf llvm-4.0.1.src.tar.xz`
 * `cd llvm-4.0.1.src/tools`
 * `svn co http://llvm.org/svn/llvm-project/cfe/tags/RELEASE_401/final clang`
 * `cd ..` # back to llvm-4.0.1.src dir
 * `mkdir build; cd build`
 * `cmake -G "Unix Makefiles" -DCMAKE_INSTALL_PREFIX=$HOME/llvm-c3 -DLLVM_TARGETS_TO_BUILD=X86 -DLIBCLANG_BUILD_STATIC=ON -DLLVM_BUILD_LLVM_DYLIB=OFF -DLLVM_INCLUDE_TESTS=OFF -DCMAKE_BUILD_TYPE=MinSizeRel ..`
 * `make -j8; make install`
 * Take a nap.
 * `cp lib/libclang.a "$HOME/llvm-c3/lib/"`
 * `export LIBCLANG_INCLUDE_PATH="$HOME/llvm-c3/tools/clang/include/:$HOME/llvm-c3/include/"`
 * `export LIBCLANG_STATIC_PATH="$HOME/llvm-c3/lib/"`
 * `export LLVM_CONFIG_PATH="$HOME/llvm-c3/bin/llvm-config"`
