Ensure docker engine is installed on host machine.

Run the following to build the image:

```
docker build -t paygo-portal/static-file-server:latest \
  --build-arg="application_json_url=https://storage.googleapis.com/paygo-portal-colorado-assistant/static-file-server.json" \
  .
```

Run the following to run the container on a local machine:

```
docker run -d -v static-files:/public paygo-portal/static-file-server:latest
```

