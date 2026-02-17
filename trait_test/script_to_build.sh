#! /bin/bash

cargo build --release
FILENAME=$(./target/release/trait_test)
# wait %1
FILE_NO_EXT=${FILENAME::-4}

magick $FILENAME $FILE_NO_EXT.png

rm $FILENAME
