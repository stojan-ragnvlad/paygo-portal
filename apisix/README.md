# Creating the `apisix.yaml` file

Run the following to build the docker image:

```
docker build -t paygo-portal/apisix \
  --build-arg="application_json_url=https://storage.cloud.google.com/paygo-portal-colorado-assistant/state-wide-app-administration.json" \
  .
```

Run the following to start the docker container:

```
docker run -d --rm -v apisix:/pp_data paygo-portal/apisix
```

# Starting the API Gateway

Run the following to pull the docker image:

```
docker pull apache/apisix:3.9.1-debian
```

Run the following to start the docker container:

```
docker run -d -v apisix:/usr/local/apisix/conf/ -p 9080:9080 \
  -e APISIX_STAND_ALONE=true apache/apisix:3.9.1-debian
```

