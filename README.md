# distreqs
A hackday project to learn the tools in the Rust-lang ecosystem for the web
This project uses rocket and redis-rs

## Rationale
This project is the server behing the idea of having clients making http requests for us.
A piece of work is obtained from a redis list and sent to a client, at a later point the clients
will send the content of doing a GET to that url back.

## Endpoints

- `GET /work` clients get a URL to make the request
- `GET /work/<url>` client gets the content for that URL
- `POST /work -d url` some client is sending us a URL to later send to normal clients
- `POST /work/content -d {"url": ..., "content": ...}` a client is replying with the data fetched in the request 
