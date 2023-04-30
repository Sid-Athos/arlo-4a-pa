FROM rust:1.68 as build

RUN USER=root cargo new --bin api_game
WORKDIR /api_game

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/api*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /api_game/target/release/api_game .

CMD ["./api_game"]
