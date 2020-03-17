FROM rust:1.42-alpine

RUN apk update

ENV APP_PATH=/capoomobi

WORKDIR ${APP_PATH}

COPY . .

RUN cargo build --release