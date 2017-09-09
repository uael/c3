# C3

Internal for the C3 project.

## Building

 * Run `cargo clean` if upgrading from a previous version of LLVM.
 * Build [LLVM 4 or 5 and Clang](http://releases.llvm.org/download.html) from source (`libclang.a` is needed, and pre-built packages won't have it).
 * Add directory containing `llvm-config` to your `PATH`, or set `LLVM_CONFIG_PATH` env variable poiting to the `llvm-config` executable file.
 * Set `LIBCLANG_INCLUDE_PATH` pointing to Clang's include directory (`<clang install dir>/clang/include/`)

### Building clang

This will require 20GB of disk space and 16GB of RAM.

 * Install cmake, subversion, build-essential, libffi-dev, libncursesw5-dev
 * `curl -LO http://releases.llvm.org/5.0.0/llvm-5.0.0.src.tar.xz`
 * `tar xf llvm-4.0.1.src.tar.xz`
 * `curl -LO http://releases.llvm.org/5.0.0/cfe-5.0.0.src.tar.xz`
 * `tar xf cfe-5.0.0.src.tar.xz`
 * `mv cfe-5.0.0.src llvm-4.0.1.src/tools/clang`
 * `cd llvm-4.0.1.src`
 * `mkdir build; cd build`
 * `cmake -G "Unix Makefiles" -DCMAKE_INSTALL_PREFIX=$HOME/llvm-c3 -DLIBCLANG_BUILD_STATIC=ON -DLLVM_BUILD_LLVM_DYLIB=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DCMAKE_BUILD_TYPE=MinSizeRel -DLLVM_POLLY_BUILD=OFF -DLLVM_BUILD_RUNTIME=OFF -DLLVM_ENABLE_TERMINFO=OFF -DLLVM_ENABLE_LIBEDIT=OFF -DLLVM_ENABLE_ZLIB=OFF -DLLVM_ENABLE_FFI=OFF ..`
 * `make -j8; make install`
 * Take a nap.
 * `cp lib/libclang.a "$HOME/llvm-c3/lib/"`
 * `export LIBCLANG_INCLUDE_PATH="$HOME/llvm-c3/tools/clang/include/:$HOME/llvm-c3/include/"`
 * `export LIBCLANG_STATIC_PATH="$HOME/llvm-c3/lib/"`
 * `export LLVM_CONFIG_PATH="$HOME/llvm-c3/bin/llvm-config"`
