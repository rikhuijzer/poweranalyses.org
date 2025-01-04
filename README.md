# poweranalyses.org

Statistical power analyses in the browser via R's nmath library and WebAssembly.

## License

The favicon is obtained from Flaticon (https://www.flaticon.com/free-icon/statistics_4064965).

## Developer notes

Going via emscripten because we link a C library.

A big thanks to https://github.com/rustwasm/team/issues/291#issuecomment-644946504 for writing down how to build a C library to WebAssembly via Rust.

For local development, checkout the `justfile`.

To see the changes live, run
```sh
$ just serve
```

To only build the site, run
```sh
$ just build
```

To run the backend tests (Rust), run

```sh
$ cargo test
```

Or use `cargo watch` to run the tests automatically when you make changes.

```sh
$ cargo watch -x "test"
```
