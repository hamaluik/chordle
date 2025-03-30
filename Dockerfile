# Build image
FROM rust AS build
WORKDIR /usr/src

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new chordle
WORKDIR /usr/src/chordle
COPY Cargo.toml Cargo.lock ./
RUN echo "fn main() {}" > build.rs
RUN cargo build --release

COPY build.rs ./
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Runtime image
FROM alpine
LABEL org.opencontainers.image.source="https://github.com/hamaluik/chordle"

RUN apk add --no-cache tzdata
ENV TZ=America/Edmonton
ENV SQLITE_DB=/data/chordle.db
ENV BIND=0.0.0.0:8080

COPY --from=build /usr/local/cargo/bin/chordle /usr/bin/chordle
ENTRYPOINT ["chordle"]
CMD ["-v"]
