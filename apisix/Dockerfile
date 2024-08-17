FROM node:22.5.1-alpine3.19

ARG application_json_url

RUN npm i -g ejs@3.1.10

RUN apk --no-cache add curl

RUN curl $application_json_url -o application.json

RUN mkdir pp_data

COPY apisix.ejs .

RUN ejs apisix.ejs -f application.json -o apisix.yaml

CMD ["mv", "apisix.yaml", "pp_data"]

