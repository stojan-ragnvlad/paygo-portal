FROM node:21.7.3-alpine3.19

RUN apk --no-cache add curl

ARG application_url

WORKDIR /build

RUN curl $application_json_url -o application.json

COPY package.json package-lock.json .

RUN npm i

COPY webpack.config.js .

COPY src src

RUN npm i -g ejs@3.1.10

RUN ejs src/App.ejs -o src/App.js -f application.json

CMD npm run build

