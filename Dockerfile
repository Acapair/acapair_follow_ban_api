FROM rust:1.77.2-alpine3.18
COPY Cargo.toml ./
COPY src ./src
RUN apk update
RUN apk add musl-dev
RUN cargo build --release
CMD [ "/target/release/acapair_follow_ban_api"]