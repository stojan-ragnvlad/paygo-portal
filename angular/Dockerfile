FROM node:22.1.0-alpine3.18

COPY web-browser-client-angular/src /web-browser-client-angular/src

COPY web-browser-client-angular/angular.json\
  web-browser-client-angular/package.json\
  web-browser-client-angular/package-lock.json\
  web-browser-client-angular/tsconfig.app.json\
  web-browser-client-angular/tsconfig.json\
  web-browser-client-angular/tsconfig.spec.json /web-browser-client-angular

COPY package.json package-lock.json /

RUN npm i

WORKDIR /web-browser-client-angular

RUN npm i

RUN npm i -g @angular/cli

RUN ng build web-browser-client-angular --output-path /public

