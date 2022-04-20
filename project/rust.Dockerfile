FROM rust:1

WORKDIR /app
COPY . .

CMD cargo run server