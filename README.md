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
 <img width="1431" alt="random-generated-shortname" src="https://github.com/AYUB-JAMA/ziply/blob/master/src/client/screenshot/random-generated-shortname.png">
  <img width="1431" alt="choosing-short-name" src="https://github.com/AYUB-JAMA/ziply/blob/master/src/client/screenshot/choosing-short-name.png">

 ## Features Missing
 * Custom domain support.
 * View, edit, delete and manage your links.
 * Stats with charts
 * description for links.

## Setup

### Manual

You need to have [Node.js](https://nodejs.org/), [MONGODB](https://www.mongodb.com) and [Rust](https://www.rust-lang.org/tools/install) installed.

1. Clone this repository or [download the latest zip](https://github.com/AJ095/Ziply).
2. Copy `.example.env` to `.env` and fill it properly ([see below](#configuration)).
Node + React
3. Install dependencies: `npm install`.
4. Run for development: `npm run dev`.
5. Run for production: `npm run build` then `npm start`.
Rust + Actix
6. Just run: `cargo run`.

