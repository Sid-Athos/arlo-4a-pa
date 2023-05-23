FROM rust:1.68 as build

RUN USER=root cargo new --bin game-manager
WORKDIR /game-manager

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/game_manager*
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=build /game-manager/target/release/game-manager .

CMD ["./game-manager"]
