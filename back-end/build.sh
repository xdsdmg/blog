#!/bin/sh -e

binary_name='back-end'

mkdir -p output/conf

cp -r conf output/
cp -r blog output/
cp run.sh output/

cargo build -r

cp ./target/release/${binary_name} output/
