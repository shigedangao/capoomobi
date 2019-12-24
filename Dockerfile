FROM rust:1.40.0-alpine

RUN apk update

ENV APP_PATH=/capoomobi

WORKDIR ${APP_PATH}

COPY . .

RUN cargo build

CMD [ "cargo run init capoomobi ../ && cargo run generate ./example" ]
