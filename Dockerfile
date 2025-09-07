FROM linuxcontainers/debian-slim:latest

#COPY Cargo.toml ./
#COPY src ./src
COPY target/release/acapair_follow_ban_api .
#RUN apk add --no-cache musl-dev curl gcc
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# ENV PATH="/root/.cargo/bin:${PATH}"
# RUN cargo build --release
# RUN cp /target/release/acapair_follow_ban_api .
# RUN cargo clean
# RUN rustup self uninstall -y
# RUN apk del musl-dev curl gcc
# RUN rm Cargo.lock
CMD [ "./acapair_follow_ban_api"]