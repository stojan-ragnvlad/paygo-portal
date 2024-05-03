FROM node:22.1.0-alpine3.18

COPY web-browser-client-angular/src angular.json package.json\
package-lock.json tsconfig.app.json tsconfig.json tsconfig.spec.json\
src

COPY package.json package-lock.json .

RUN npm i

RUN cd src

RUN npm i

RUN npm i -g @angular/cli

RUN ng build public --output-path ../public

