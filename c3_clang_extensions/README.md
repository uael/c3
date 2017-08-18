# C3

Internal for the C3 project.

## Building

 * Build [LLVM 4 and Clang](http://releases.llvm.org/download.html) from source (`libclang.a` is needed, and pre-built packages won't have it).
 * Add directory containing `llvm-config` to your `PATH`, or set `LLVM_CONFIG_PATH` env variable poiting to the `llvm-config` executable file.
 * Set `LIBCLANG_INCLUDE_PATH` pointing to Clang's include directory (`<clang install dir>/clang/include/`)

### Building clang

 * Install cmake, subversion, build-essential
 * `curl -LO http://releases.llvm.org/4.0.1/llvm-4.0.1.src.tar.xz`
 * `tar xf llvm-4.0.1.src.tar.xz`
 * `cd llvm-4.0.1.src/tools`
 * `svn co http://llvm.org/svn/llvm-project/cfe/tags/RELEASE_401/final clang`
 * `cd clang/tools`
 * `svn co http://llvm.org/svn/llvm-project/clang-tools-extra/tags/RELEASE_401/final extra`
 * `cd ../../..`
 * `mkdir build; cd build`
 * Optionally
     * `export LLVM_TARGETS_TO_BUILD=x86_64` (or your platform)
     * `export LLVM_BUILD_TOOLS=OFF`
     * `export LLVM_INCLUDE_TESTS=OFF`
     * `export LLVM_BUILD_LLVM_DYLIB=OFF`
 * `cmake -G "Unix Makefiles" ..`
 * `make clang -j6`
 * Take a nap.
