# `maize`: Strongly typed JIT for PTX kernels

`maize` is an effort to make it easier (possible?)
to write CUDA kernels in Rust, using a tracing JIT
system built on LLVM and [inkwell](https://github.com/TheDan64/inkwell).

The hope is to eventually make writing even complex
kernels fast and easy (easier?), while using a strong
type system to increase confidence in their correctness.

There is currently a (meaningfully) incorrect pipelined BF16
kernel available as an example in `src/main.rs`.
It deliberately ignores edge tiles, only works for 4k by 4k
matrices, copies without swizzle to shared memory, but it
should give a sense for what the system as a whole is capable of.
It produces PTX visible in [example_ptx/gen.ptx](./example_ptx/gen.ptx).
Alternatively, have a look at the project [overview](./media/overview.md)
or [motivation](./media/motivation.md).
