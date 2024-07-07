import express from 'express';
import { exec } from 'child_process';

const server = express();

server.use(express.text());

server.use(express.static('public/'));

server.post('/query', (request, response) => {
  console.log(request.body);

  exec(
    `sh executables/main ${request.body}`,
    (error, standardOut, standardError) => {
      response.send({ error, standardOut, standardError });
    }
  );
});

server.listen(80);

