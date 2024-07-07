if [ "$PP_BUILD_TYPE" = "wasm" ]
then
  wasm-pack build --dev --target web -d package

  mkdir -p dist && cp -R package/* dist
elif [ "$PP_BUILD_TYPE" = "executable" ]
then
  cargo build --bins

  ls

  exit 0
fi

