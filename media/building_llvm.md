# Building LLVM

LLVM is a huge project, and taking a dependency on it without bundling it
can be a substantial burden. As this project is still in its early days,
I'm mostly focused on getting things working rather than maximizing
compatibility. My _guess_ is that things won't be much worse on a different
build (e.g. package manager), but for the avoidance of doubt, I do the following:

I currently build LLVM from [source](https://github.com/llvm/llvm-project) at tag
`llvmorg-21.1.8`. I build this on MacOS (`aarch64-apple-darwin`) using the
following configuration
```bash
mkdir -p ./install_prefix
cmake \
	-S llvm \
	-B build \
	-G Ninja \
	-DLLVM_TARGETS_TO_BUILD="NVPTX" \
	-DCMAKE_INSTALL_PREFIX=./install_prefix \
	-DCMAKE_BUILD_TYPE=Release
ninja -C build install -v
```

If you don't have `ninja`, you should be able to remove the `-G Ninja` and run `make -C build install`
instead. I believe `cmake` is roughly required for an LLVM build.

## Using LLVM (`llvm-sys`)

Once you have built a copy of LLVM, you just need to point `llvm-sys` at it and enable the
right features. E.g. in my own development environment, I have a `.cargo/config.toml` containing
```toml
[env]
LLVM_SYS_211_PREFIX = "~/development/llvm-21/install_prefix"
```

with a matching declaration in `maize_core`'s `Cargo.toml`:
```toml
[dependencies]
inkwell = { version = "0.8.0", default-features = false, features = ["target-nvptx", "llvm21-1"] }
```
