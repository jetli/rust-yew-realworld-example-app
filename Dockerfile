FROM rust:latest

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install trunk
wget -qO- https://github.com/thedodd/trunk/releases/download/v0.17.2/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

WORKDIR /usr/src/conduit-wasm

COPY ./crates/conduit-wasm .

COPY .env.example .env

EXPOSE 8080

CMD [ "trunk", "serve" ]
