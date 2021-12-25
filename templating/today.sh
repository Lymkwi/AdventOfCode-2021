#!/bin/sh
today=$(date +%d)
cp -r 'dayZZ' "day$today"
sed -i "s/ZZ/$today/g" $(find "day$today" -type f)
echo -e "\n[dependencies.day$today]\npath = \"day$today\"" >> Cargo.toml
