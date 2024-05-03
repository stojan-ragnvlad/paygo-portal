import express from 'express';
import { Request, Response } from 'express';

const server = express();

server.use(express.static('public/browser'));

server.listen(80);

