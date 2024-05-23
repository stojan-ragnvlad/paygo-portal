Run the following from a terminal in the same directory as this read me file:

```
docker build -t paygo-portal/web-project:latest . && mkdir -p dist
```

then, run the following:

```
docker run -v ./dist:/dist paygo-portal/web-project:latest
```

