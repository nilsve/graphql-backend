FROM lukemathwalker/cargo-chef:latest as chef

WORKDIR /app

FROM chef AS planner

RUN mkdir -p orm/src
RUN mkdir -p server/src
RUN echo 'fn main() { panic!("Dummy Image Called!")}' > ./orm/src/lib.rs
RUN echo 'fn main() { panic!("Dummy Image Called!")}' > ./server/src/main.rs

COPY ["Cargo.toml", "Cargo.lock", "./"]
COPY ["./orm/Cargo.toml", "./orm/"]
COPY ["./server/Cargo.toml", "./server/"]

RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release

COPY . .

RUN cargo build --release
RUN mv ./target/release/server ./app

FROM node as frontend_builder

WORKDIR /app

COPY ./frontend/package.json .

RUN npm install

COPY ./frontend .

RUN npm run build

FROM debian:stable-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates

WORKDIR /usr/local/bin
COPY --from=builder /app/app .
COPY --from=frontend_builder /app/dist ./static

EXPOSE 8080

ENTRYPOINT ["./app"]
