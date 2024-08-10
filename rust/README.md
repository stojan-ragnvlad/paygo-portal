* https://docs.rs/qrcode-generator/latest/qrcode_generator/fn.to_png_to_file.html
* https://doc.rust-lang.org/std/hash/index.html
* https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm

## Build Steps

* Run to build the image:

```
docker build --build-arg="PP_BUILD_TYPE=wasm" -t paygo-portal/rust:latest .
```

* Run to build the project:

```
docker run --rm -e PP_BUILD_TYPE=wasm -v ./package:/rust_build/dist\
  paygo-portal/rust:latest
```

