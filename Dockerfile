FROM rust:latest
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo install dioxus-cli
COPY ./asset/ ./asset/
COPY ./src/ ./src/
COPY ./config.json ./config.json
RUN dx build --release
EXPOSE 8080
CMD ["dx", "serve", "--release", "--fullstack", "--port", "8080"]