FROM rust:1.70-slim-buster as build
RUN USER=root cargo new --bin imu-backend

WORKDIR /imu-backend
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN cargo build --release
FROM debian:buster-slim

COPY --from=build /imu-backend/target/release/imu-backend .
EXPOSE 8080
CMD ["./imu-backend"]
