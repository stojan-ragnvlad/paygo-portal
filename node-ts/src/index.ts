import express from 'express';
import { Request, Response } from 'express';

const server = express();

server.use(express.static('public'));

server.get('/', (request: express.Request, response: express.Response) => {
  response.send('Hello World!');
});

server.listen(80);

