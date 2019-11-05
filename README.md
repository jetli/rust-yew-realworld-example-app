# ![RealWorld Example App](logo.png)

[![RealWorld Frontend](https://img.shields.io/badge/realworld-frontend-%23783578.svg)](http://realworld.io)

> ### [Rust] + [Yew] + [WebAssembly] codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld] spec and API.


### [Demo]&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld]


This codebase was created to demonstrate a fully fledged [WebAssembly] web application built with [Yew] including CRUD operations, authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the [Yew] community styleguides & best practices.

For more information on how to this works with other frontends/backends, head over to the [RealWorld] repo.


# How it works

This is an application written in [Rust] that utilizes [Yew] and [WebAssembly] for developing the frontend web app that powers the RealWorld application.

You can view a full list of crates being used in [Cargo.toml], but here are some of the main ones of note:

* [Yew] - a modern Rust framework for creating multi-threaded frontend apps with WebAssembly.
* [yew-router] - a routing library for the [Yew] framework.

# Getting started

You can view a live demo over at [Demo]

* Install [Rust]
* Install [cargo-web]
  ```
  cargo install cargo-web
  ```
* Build and develop
  ```
  cargo web start -p conduit-wasm
  ```
  You can visit `http://[::1]:8000` in your browser now.
* Build and release
  ```
  cargo web deploy -p conduit-wasm --release
  ```
  You should find static files at `target/deploy` folder now.

[Rust]: https://www.rust-lang.org/
[Yew]: https://github.com/yewstack/yew
[RealWorld]: https://github.com/gothinkster/realworld
[Demo]:https://jetli.github.io/rust-yew-realworld-example-app/
[WebAssembly]: https://webassembly.org
[cargo-web]: https://github.com/koute/cargo-web
[Cargo.toml]: ./crates/conduit-wasm/Cargo.toml
[yew-router]: https://github.com/yewstack/yew_router