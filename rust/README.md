* https://docs.rs/qrcode-generator/latest/qrcode_generator/fn.to_png_to_file.html
* https://doc.rust-lang.org/std/hash/index.html
* https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm

Run the following to install wasm pack:

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

You must have clang and cmake installed!
* https://cmake.org/download/
  * Download the sh install script and add the binaries to your path!
* Run `sudo apt install clang`.

Install enscripten:
* https://www.hellorust.com/setup/emscripten/
* https://emscripten.org/docs/getting_started/downloads.html#
* `./emsdk install 1.38.45 && ./emsdk activate 1.38.45`
* Add this to the `.bashrc` file:
```
export PATH="/home/stojan/.local/emsdk/upstream/emscripten/emcc:$PATH"
```

Install this:

```sudo apt install python-is-python3```

