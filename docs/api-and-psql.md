```
sudo docker run -d -i --name some-postgres -h 127.0.0.1 -p 8081:5432 -e POSTGRES_PASSWORD=mysecretpassword postgres:16.3-alpine3.20
```

* `-e POSTGRES_HOST_AUTH_METHOD=trust`
* `docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' sqltutorial`

```
docker run -p 8080:80 \
  -e 'PGADMIN_DEFAULT_EMAIL=user@domain.com' \
  -e 'PGADMIN_DEFAULT_PASSWORD=SuperSecret' \
  -d dpage/pgadmin4
```
