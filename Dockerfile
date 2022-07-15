FROM rust:alpine
COPY . /home
RUN Cargo run --release