FROM node:22-alpine3.19

COPY webpack.config.js package.json package-lock.json .

COPY src src

RUN npm i

RUN npm run build

CMD exit 0

