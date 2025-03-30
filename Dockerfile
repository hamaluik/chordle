# Build image
FROM rust AS build
WORKDIR /usr/src

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new chordle
WORKDIR /usr/src/chordle
COPY Cargo.toml Cargo.lock ./
ENV TZ=America/Edmonton
RUN cargo build --release --target x86_64-unknown-linux-musl

COPY build.rs ./
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime image
FROM alpine
LABEL org.opencontainers.image.source="https://github.com/hamaluik/chordle"

RUN apk add --no-cache tzdata
ENV TZ=America/Edmonton
ENV SQLITE_DB=/data/chordle.db
ENV BIND=0.0.0.0:8080

COPY --from=build /usr/src/chordle/target/x86_64-unknown-linux-musl/release/chordle /usr/bin/chordle
ENTRYPOINT ["chordle"]
CMD ["-v"]
