#!/bin/bash
BLUE="\033[34m"
GREEN="\033[32m"
RED="\033[31m"
PURPLE="\033[35m"
LIME="\033[36m"
RESET="\033[0m"

NAME="minesweeper-rs"

printf "\n ==> ${PURPLE}Removing artifacts and setting up workspace${RESET}\n\n"
rm -rvf release
mkdir -p release

printf "\n ==> ${BLUE}Building a release version of the project${RESET}\n\n"
cargo build --release
cargo build --release --target x86_64-pc-windows-gnu

cp target/release/"$NAME" release/"$NAME"_linux64
cp target/x86_64-pc-windows-gnu/release/"$NAME".exe release/"$NAME"_windows64.exe

printf "\n ==> ${GREEN}Stripping binaries${RESET}\n\n"
strip release/"$NAME"_linux64 release/"$NAME"_windows64.exe

printf "\n ==> ${RED}Compressing binaries${RESET}\n\n"
upx --best --lzma release/"$NAME"_linux64 # release/"$NAME"_windows64.exe (Windows defender flags the UPX stub as a threat)

printf "\n ==> ${LIME}All done.${RESET}\n\n"
