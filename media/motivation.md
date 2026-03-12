# Why build a JIT?

I've been writing CUDA kernels for my day job for a while now. While I quite
enjoy getting GPUs to run quickly, I've experienced a pretty consistent headache
associated with using a C++ dialect. In particular, I (still) don't enjoy
- How overloading and header includes cause virtually every LSP I've tried
to slow to a crawl (`clangd` is _almost_ usable, but it too eventually
gives up with enough templating). `rust-analyzer` seems to run vastly faster,
and made writing code much, much easier.
- How in highly generic code, there is no way to express what an `auto` type
can even be -- is it a variable? A callable? With how many arguments? What are
their types? Is it something just passed around so you can `decltype()` it for
template substitution? The same goes for auto-return-type functions.
- `nvcc`, `CuTe DSL`, and a lot of the standard tooling is fairly slow, which
makes compiling in a loop a hassle. I didn't know if I could solve this, but it
seemed worth a try.

To ensure that any ultimate executables remained decently quick, I ended up
landing on Rust. I tried a bit ago to try to use native `rustc` compilation to
generate kernels, and found it was quite messy (see
[averyparr/gpu_native](https://github.com/averyparr/gpu_native)). Unfortunately,
this ran into limitations with `rustc`, namely that it doesn't support address
spaces (key for share memory on GPUs, without which you can do very little).
While I was able to get a very limited implementation of address-space-tagged
statics working in a dev branch for `rustc`, I ultimately wanted to make faster
progress.

## Enter `inkwell`:

So I did what everyone else seems to do these days and built on top of the LLVM project. There's a really nice crate by Dan Kolsoi called
[inkwell](https://github.com/TheDan64/inkwell) which provides a more strongly
typed interface to the underlying LLVM library, and which made it possible to
deal with the more onerous parts of LLVM while keeping its underlying codegen
capabilities.
