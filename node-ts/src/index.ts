import express from 'express';
import { Request, Response } from 'express';

const server = express();

server.use(express.static('public/'));

server.listen(80);

