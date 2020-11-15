# Context-Free Grammar Parser Example[![Build Status](https://travis-ci.com/kgrech/lalrpop-expression.svg?branch=webassembly)](https://travis-ci.com/kgrech/lalrpop-expression)

Implementation formal grammar parser using [Rust](https://www.rust-lang.org/) 
and [Lalrpop](https://github.com/lalrpop/lalrpop) crate with a support of [web-assembly](https://rustwasm.github.io/)!

View more on [YouTube](https://www.youtube.com/playlist?list=PLc493xNgYTbJDNa5MNGScdusl4LwOlnvd)

## Webassembly dependencies setup
Follow the official rustwasm [tutorial](https://rustwasm.github.io/docs/book/game-of-life/setup.html)

## Compile web-assembly package
```
wasm-pack build
```
## Run web application
```
cd www
npm install
npm start
```
## Run tests
```
cargo test
```
