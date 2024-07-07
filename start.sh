docker volume prune -f

docker image prune -f

docker volume rm static-files

docker volume create static-files

docker volume rm web-src

docker volume create web-src

cd rust

docker build -t paygo-portal/rust:latest .

docker run --rm -e PP_BUILD_TYPE=wasm -v web-src:/rust_build/dist\
  paygo-portal/rust:latest

docker run --rm -e PP_BUILD_TYPE=executable -v \
  executables:/rust_build/target/debug paygo-portal/rust:latest

cd ../web-browser

docker build -t paygo-portal/web-project:latest .

docker run --rm \
  -v static-files:/build/dist \
  -v web-src:/build/lib \
  paygo-portal/web-project:latest

cd ../node-ts

docker build -t paygo-portal/node-ts:latest .

docker run --init --rm -p 127.0.0.1:8080:80/tcp -v static-files:/public\
  -v executables:/executables paygo-portal/node-ts:latest

docker volume prune -f

docker image prune -f

