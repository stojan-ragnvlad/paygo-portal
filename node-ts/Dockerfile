FROM node:21.7.3-alpine3.19

ARG application_json_url

RUN npm i -g ejs@3.1.10

RUN apk --no-cache add curl

RUN curl $application_json_url -o application.json

COPY package.json package-lock.json .

RUN npm i

COPY tsconfig.json .

RUN mkdir src

COPY src/* src

RUN ejs src/index.ejs -f application.json -o src/index.ts

RUN mkdir dist

RUN npx tsc

CMD ["node", "dist/index.js"]

