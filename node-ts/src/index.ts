import express from 'express';
import { exec } from 'child_process';
import net from 'net';

const server = express();

server.use(express.text());

server.use(express.static('public/'));

server.post('/query', (request, response) => {
  const client = net.createConnection({ host: 'tcp-server', port: 3001 }, () => {
    client.write(Buffer.from(request.body), () => console.log('hahaha'));
  });

  client.on('data', () => {
    client.destroy();
  });

  client.on('error', (error) => console.log('error!', error));
});

server.listen(80);

