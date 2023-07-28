# Ziply
ziply allows you to create shorter and easier-to-share links. Start using Ziply today for swift and efficient link shortening!

## Stack
- Actix-web (Web server framework)
- React (UI library)
- MongoDB (database)

## Key Features

- Free and open source.
- Custom URL-Name for shortened links
- RESTful API.
## Shorten the url by the supplied shortname
## Shorten the url by generating a random name in case shortname isn't supplied
  <img width="1431" alt="choosing-short-name" src="https://github.com/AYUB-JAMA/molurl/assets/42579401/72a1ef9a-7ce1-4392-90ff-317ba47ed084">
  <img width="1431" alt="random-generated-shortname" src="https://github.com/AYUB-JAMA/molurl/assets/42579401/6f36cd43-7fb1-4aba-9823-78965d65c717">

## Setup

### Manual

You need to have [Node.js](https://nodejs.org/), [MONGODB](https://www.mongodb.com) and [Rust](https://www.rust-lang.org/tools/install) installed.

1. Clone this repository or [download the latest zip](https://github.com/AYUB-JAMA/Ziply).
2. Copy `.example.env` to `.env` and fill it properly ([see below](#configuration)).
Node + React
3. Install dependencies: `npm install`.
4. Run for development: `npm run dev`.
5. Run for production: `npm run build` then `npm start`.
Rust + Actix
6. Just run: `cargo run`.

