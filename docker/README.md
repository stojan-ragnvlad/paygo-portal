Ensure docker engine is installed on host machine.

Run the following to build the image:

```
docker build -t node-ts-project:latest .
```

Run the following to run the container on a local machine:

```
docker run -d angular-project:latest
```

```
docker cp container_name:public .
```

```
docker run -d -v rust-volume:/public rust-project:latest
```

```
docker cp container_name:public .
```

```
docker run -d -p 127.0.0.1:8080:80/tcp -v angular-volume:/public\
 -v rust-volume:/public node-ts-project:latest
```

