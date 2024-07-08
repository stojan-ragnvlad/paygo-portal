FROM node:21.7.3-alpine3.19

WORKDIR build

COPY package.json package-lock.json .

RUN npm i

COPY webpack.config.js .

COPY src src

CMD npm run build

