FROM node:22-alpine3.19

COPY webpack.config.js package.json package-lock.json .

COPY src src

RUN npm i

CMD npm run build

