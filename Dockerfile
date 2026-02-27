# Stage 1: Build the frontend
FROM oven/bun:1.3.9 AS client
WORKDIR /app
COPY ./client/package.json ./
COPY ./client/bun.lock ./
RUN bun install
COPY ./client/ .
RUN bun run build

# Stage 2: Build the application
FROM rust:1.92-alpine AS server
WORKDIR /app
RUN apk add openssl-dev openssl-libs-static pkgconfig
COPY ./server/ .
COPY --from=client /app/build/ /app/client-static/
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release --bin gitmio && \
    cp target/release/gitmio ./gitmio

# Stage 3: Create the final runtime image
FROM alpine:3.22 AS runtime
COPY --from=server /app/gitmio /app/gitmio

EXPOSE 8080

CMD ["/app/gitmio"]
