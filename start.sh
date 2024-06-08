docker stop $(docker ps -a -q)

docker rm $(docker ps -a -q)

docker volume prune -f

docker image prune -f

docker volume create static-files

cd rust

docker build -t paygo-portal/rust:latest .

docker image prune -f

docker run --rm -v static-files:/package paygo-portal/rust:latest

cd ../web-browser

docker build -t paygo-portal/web-project:latest .

docker image prune -f

docker run --rm -v static-files:/dist paygo-portal/web-project:latest

cd ../node-ts

docker build -t node-ts-project:latest .

docker image prune -f

docker run -d -p 127.0.0.1:8080:80/tcp -v static-files:/public\
 node-ts-project:latest

