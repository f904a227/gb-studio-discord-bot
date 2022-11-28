FROM docker.io/rust:1.65 as builder
WORKDIR /usr/src/gb-studio-discord-bot
COPY . .
RUN cargo install --locked --path .

FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/gb-studio-discord-bot /usr/local/bin/gb-studio-discord-bot
CMD ["gb-studio-discord-bot"]
