# Future Work

This is a rough list of all the parts of the project I would ultimately like to work on:

- Higher level abstractions for common tile-based operations? Avoiding e.g.
loops, but trying to ensure minimal runtime overhead
- Support all the intrinsics. Right now, we support most synchronous MMA
intrinsics, most transcendental float functions, cp.async and a few others,
but I'd like to have a full suite. Seems a shame LLVM makes them available
for no one to use them.
- LLVM is missing the ability to lower some some intrinsics, like
`llvm.nvvm.mma.m16n8k16.row.col.f16.e4m3.e4m3.f16`, and I should probably
get it to do that. Also, `llvm.vector.reduce.{and/or}` is kind of bad
for PTX. There's work to do there.
- Meaningfully support operations newer than the good ol' 3090 can actually
run. The complex synchronization patterns is almost certainly where a nice
type system will shine.
- Work out a better way to deal with more complex layouts -- right now,
it's very easy to break something by forgetting to align an MMA op with
a global copy op.
- Swap away from purely reflection based primitives towards a complete
reflection based approach: we should use `Val<'a, f32>`, not `Val<'a, F32>`.
- Expand `kernel_assert` to allow for formatting strings like `kernel_print`
- Provide fp8 support -- though they're going to be opaque `u8` types.
- Warp reduction/shuffle support! This is a matter of translating algorithm
into implementation; the reduction is quite elegant.
- Loop unrolling supported properly?
- Try out compiling for aarch64 or amdgpu?
- Make it simple to call non-inlined functions?
