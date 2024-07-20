if [ "$PP_BUILD_TYPE" = "wasm" ]
then
  mkdir -p dist && cp -R package/* dist
elif [ "$PP_BUILD_TYPE" = "executable" ]
then
  ./target/debug/main
fi

