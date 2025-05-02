# hello-axum

This repo is built with the Axum framework on top of the Hyper HTTP library and Tokio async runtime
and attempts to employ storage with Redis and Serde.
`axum` is a web application framework that focuses on ergonomics and modularity.

[![Crates.io](https://img.shields.io/crates/v/axum)](https://crates.io/crates/axum)
[![Documentation](https://docs.rs/axum/badge.svg)][docs]

More information about this crate can be found in the [crate documentation][docs].
You can find this [example][readme-example] as well as other example projects in
the [example directory][examples].

See the [crate documentation][docs] for way more examples.

## Performance

`axum` is a relatively thin layer on top of [`hyper`] and adds very little
overhead. So `axum`'s performance is comparable to [`hyper`]. You can find
benchmarks [here](https://github.com/programatik29/rust-web-benchmarks) and
[here](https://web-frameworks-benchmark.netlify.app/result?l=rust).

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in
100% safe Rust.

## Minimum supported Rust version

axum's MSRV is 1.75.

[docs]: https://docs.rs/axum
[readme-example]: https://github.com/tokio-rs/axum/tree/main/examples/readme
[examples]: https://github.com/tokio-rs/axum/tree/main/examples
[`hyper`]: https://crates.io/crates/hyper
