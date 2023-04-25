#!/usr/bin/env sh

# install rust if missing
if ! command -v rustup &> /dev/null
then
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi

# install pnpm if missing
if ! command -v pnpm &> /dev/null
then
	curl -fsSL https://get.pnpm.io/install.sh | sh -
fi

# install global dependencies
pnpm add -g @bufbuild/buf
