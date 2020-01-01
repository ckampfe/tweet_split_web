#!/usr/bin/env sh

wasm-pack build --target web
rollup ./main.js --format iife --file ./pkg/bundle.js

# mkdir build
cp index.html build
# cp app.css build
# cp phoenix.css build
cp -r pkg build
