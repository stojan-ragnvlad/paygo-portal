Ensure docker engine is installed on host machine.

Run the following to build the image:

```
docker build -t node-ts-project:latest .
```

Run the following to run the container on a local machine:

```
docker run -d -p 127.0.0.1:8080:80/tcp node-ts-project:latest
```

