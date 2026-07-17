#!/bin/sh
if ! command -v x86_64-w64-mingw32-gcc >/dev/null 2>&1; then
echo "Error: MinGW linker (mingw-w64-gcc) is missing."
echo "Install it on Arch: sudo pacman -S mingw-w64-gcc"
exit 1
fi

rustup target add x86_64-pc-windows-gnu

export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc

cargo build --release --target x86_64-pc-windows-gnu