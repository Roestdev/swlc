#! /bin/bash

# check if 'swlc_core/src/model_content.rs' exits.
# If not then create an empty version in order to be able to build!
./touch_model_content.sh

# clean-up
cargo clean

#  build swlc_gen
cargo build -p swlc_gen

# generate 'model_content.rs' from text files
cargo run -p swlc_gen

# copy the generated file to 'swlc_core/src'
cargo run -p swlc_gen c

# build swlc_cli
# cargo build -p swlc_cli

# build swlc_gui
# cargo build -p swlc_gui

