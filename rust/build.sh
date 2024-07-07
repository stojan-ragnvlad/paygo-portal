if [ "$PP_BUILD_TYPE" = "wasm" ]
then
  cargo add wasm-bindgen@0.2.92

  cargo add regex@1.10.5

  cargo add csv@1.3.0

  wasm-pack build --dev --target web -d package

  mkdir -p dist && cp -R package/* dist
elif [ "$PP_BUILD_TYPE" = "executable" ]
then
  cargo add postgres@0.19.7

  cargo build --bins
fi

