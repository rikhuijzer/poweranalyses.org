# `script/`

This folder contains some helper scripts that are useful during development.

- `build.sh`: Compile the Rust code to WebAssembly and copy the result to the public directory.
- `entr-build.sh`: Build the project when a file changes.
- `entr-frontend.sh`: Move the front end files when a file changes.
- `public.sh`: Copy the files to the public directory.
- `serve.sh`: Serve the website locally with live reload.

For running the tests when a file changes, use `cargo watch`:

```sh
$ cargo install cargo-watch

$ cargo watch -x test
```

To then run only one test called `some_test`, use:

```sh
$ cargo watch -x 'test some_test'
```
