FROM rust:latest

WORKDIR /usr/src/rcc
COPY . .

CMD ["cargo", "t"]
