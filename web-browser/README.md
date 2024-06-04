Run the following from a terminal in the same directory as this read me file:

```
mkdir -p dist && rm -rf ./dist/*
```

then, run the following:

```
docker build -t paygo-portal/web-project:latest .
```

then, run the following:

```
docker run -v ./dist:/dist paygo-portal/web-project:latest
```

