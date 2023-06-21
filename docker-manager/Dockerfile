FROM rust:1.68 as build

RUN USER=root cargo new --bin docker-manager
WORKDIR /docker-manager

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src-old/*.rs
COPY src-old ./src

RUN rm ./target/release/deps/docker_manager*
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt update
RUN apt install apt-transport-https ca-certificates curl gnupg lsb-release -y
RUN install -m 0755 -d /etc/apt/keyrings
RUN curl -fsSL https://download.docker.com/linux/debian/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
RUN chmod a+r /etc/apt/keyrings/docker.gpg

RUN echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/debian $(lsb_release -cs) stable" > /etc/apt/sources.list.d/docker.list

RUN apt update
RUN apt -y install docker-ce docker-ce-cli containerd.io
COPY --from=build /docker-manager/target/release/docker-manager .

CMD ["./docker-manager"]
