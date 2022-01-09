FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features sqlite

WORKDIR ./usr/src/app
VOLUME ["/usr/local/cargo"]
EXPOSE 8000