#!/bin/sh

cp ./src/main.rs main_content

echo "fn main() {}" > ./src/main.rs

cargo build --verbose

mv main_content ./src/main.rs

