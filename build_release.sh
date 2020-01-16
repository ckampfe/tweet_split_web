#!/usr/bin/env sh

wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js

cp index.html build

echo "Stripping wasm binary"
wasm-strip pkg/tweet_split_web_bg.wasm
echo "Stripped wasm binary"
echo "Optimizing wasm"
wasm-opt -o pkg/tweet_split_web_bg.wasm -Oz pkg/tweet_split_web_bg.wasm
echo "wasm optimized"
cp -r pkg build
