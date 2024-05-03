FROM node:21.7.3-alpine3.19

COPY package.json package-lock.json tsconfig.json src/* .

RUN npm i

RUN npx tsc

CMD node index.js

