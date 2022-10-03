#!/bin/bash

rustc +nightly \
    ${1:-./test.rs} \
    --target riscv32imc-unknown-none-elf \
    -o ./test.elf \
    -C lto=fat \
    -C relocation-model=static \
    -C opt-level=z \
    -C panic=abort \
    -C linker-flavor=ld.lld \
&& \
llvm-objdump -t ./test.elf | grep print_full_process > /dev/null \
