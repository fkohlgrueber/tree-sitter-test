# tree-sitter-test

This project shows how tree-sitter can be used in a Rust client-side web app. This currently (as of 08/2019) requires using emscripten.

**[Live Demo](https://fkohlgrueber.github.io/tree-sitter-test/)**

## Usage

```
$ git clone git@github.com:fkohlgrueber/tree-sitter-test.git --recurse-submodules
$ cd tree-sitter-test/
$ cargo web start --target wasm32-unknown-emscripten
```

## Update demo

```
git checkout gh-pages
git merge master
cargo web deploy --release --target wasm32-unknown-emscripten -o .
git commit -am "update demo"
git push
```
