FROM rust:latest

# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get update && apt-get install nodejs

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /usr/src/conduit-wasm

COPY ./crates/conduit-wasm .

COPY .env.example .env

RUN npm install

EXPOSE 8000

CMD [ "npm", "start" ]
