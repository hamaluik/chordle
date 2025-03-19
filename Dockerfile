FROM rust AS build
WORKDIR /usr/src

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new chordle
WORKDIR /usr/src/chordle
COPY Cargo.toml Cargo.lock build.rs ./
RUN cargo build --release

COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine
LABEL org.opencontainers.image.source="https://github.com/hamaluik/chordle"
COPY --from=build /usr/local/cargo/bin/chordle /usr/bin/chordle
USER 1000
ENTRYPOINT ["/usr/bin/chordle"]
