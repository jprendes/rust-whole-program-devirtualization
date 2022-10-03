# rust-whole-program-devirtualization

Reproduction script for whole program devirtualization issue in Rust.

## Description

The example in this repo shows an optimization opportunity for the Rust compiler motivated by reducing the binary size of [TockOS](https://github.com/tock/tock).

In many places TockOS uses traits for which there is only ever one implementation. The reason for this is that a second implementation of some components is used during testing to create test mocks.

In particular, a debug feature of TockOS prints the process memory layout to the console. This is done with the `print_full_process` method of the `Process` trait. The implementation of this method takes a reasonable amount of binary space. However, when building in production mode this method implementation is still present in the final binary, even when no other part of the code base is using it.

Whole program devirtualization is an optimization step where virtual interfaces with only one implementation are devirtualized, and the virtual calls are converted to static calls. The advantage of this optimization is two folds:
* The compiler would identify the unused function and elimate it.
* The runtime performance would improve by avoiding the unnecesary vtable lookup.

Clang (and hence LLVM) supports whole-program-devirtualization is the generated LLVM-IR is properly annotated:
* See `-fwhole-program-vtables` in [](https://clang.llvm.org/docs/ClangCommandLineReference.html)
* There's an [open issue](https://github.com/rust-lang/rust/issues/68262) in Rust's GitHub for what seems to be the same problem, but with a different reproduction steps.

## Running

Run the `test.sh` script
```bash
./test.sh && echo "program not optimized" || echoi "program optimized"
```