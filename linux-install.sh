#!/usr/bin/env bash

set -e 

pushd () { command pushd "$@" > /dev/null; }
popd () { command popd "$@" > /dev/null; }

cargo build -r;  # release build

pushd target/release/;

dest="/usr/local/bin";
sudo cp ./stb "$dest/stb";
echo "moved executable to $dest/stb";

popd;

# exporting /usr/loacl/bin 
pushd $HOME;

if [[ ":$PATH:" != *":$dest:"* ]]; then
    echo "export PATH=$PATH:$dest" >> "$HOME/.bashrc";
    source "$HOME/.bashrc";
    echo "binary path updated. You can now run 'stb' from any location."
else
    echo "The bin directory is already in the PATH. You can now run 'stb' from any location."
fi

popd


