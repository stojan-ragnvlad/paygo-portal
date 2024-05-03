To update the typescript definition of the application schema, run the
following command:

```
npx json2ts ../json-schema-validator/schemas/application-schema.json\
 application-schema.ts
```

Run the following to build the image:

```
docker build -t angular-project:latest .
```

Run the following to run the container on a local machine:

```
docker run -d -v static-files:/public angular-project:latest
```

