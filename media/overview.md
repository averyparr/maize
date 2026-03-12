# Design

`maize` is a tracing just-in-time (JIT) compiler. I intended it to essentially
do the following:
- Allow users to declare a function
- Temporarily access (mostly symbolic) variables representing
the arguments to the function
- Write down what they want the function to do in terms of
operations on those variables
- Use LLVM to turn this set of operations into runnable code
- While hopefully still upholding Rust's safety guarantees. See [upholding_safety.md](./upholding_safety.md).

## Requirements

`maize` requires an installation of LLVM capable of compiling
for your preferred target (in my case, PTX). It also requires
`inkwell`, which is more readily discoverable on crates.io.
This can be a decently large hurdle, but I'm not aware of a way
around it: if one wants to just-in-time compile their code,
they should probably have a compiler. If that's something new to
you, take a look at [building_llvm.md](./building_llvm.md)

## Values and Tracing

As a result, the fundamental type is that of the `Val<'a, T>`.
It holds a reference (hence the `'a`) to the underlying
code generation context (LLVM context + module + function).
`T` is typically required to implement several traits which
essentially require that it be able to declare its own LLVM
type, as well as create trivial values (e.g. zeroed). The
`Val` doesn't actually hold a `T` (only an LLVM value), but
we need it to keep around the type information.

I generally add additional functionality through new traits for `T`:
e.g. `SizedTy` for types which have a compile-time known size.
These traits end up _looking_ a bit strange (e.g. `MathTy` has
a `fn neg<'a>(val: Val<'a, Self>) -> Val<'a, Self>`), but this
ultimately allows for fundamental impls on `Val` for downstream
ease-of-use.

When someone performs an operation on values (e.g. adding them),
we typically do something like the following:
- Require that the type "knows how to add itself" -- e.g. `MathTy`,
- Take the raw LLVM values involved and pass them to this function
- Wrap the returned value in a strongly typed `Val` wrapper

In this way, operations on `Val`ues are registered with LLVM, and
we can ultimately use all these instructions to build out a function.

## Performance

As of now, I mostly rely on LLVM to perform optimizations.
Nonetheless, there are several key [performance](./performance.md)
considerations I've kept in mind throughout. The _hope_ is that
you should be able to write relatively simple Rust code and have
nearly-optimal PTX come out.

## Can it compile to something other than PTX?

My _hope_ is that something like what I've worked on here is
usable more widely; as far as I'm aware, there's not much JIT
activity going on in Rust, which makes sense (it's an AOT
compiled language, of course), but it's a useful tool to have.
That said, I've made just about no efforts

## Future work?

I've written down a few [future plans](./ideas_for_the_future.md)
if I end up with time to work on them.
