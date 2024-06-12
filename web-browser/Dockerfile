FROM node:21.7.3-alpine3.19

WORKDIR build

COPY webpack.config.js package.json package-lock.json .

COPY src src

RUN npm i

CMD npm run build

