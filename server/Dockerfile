FROM rust:1.59 as builder
WORKDIR rust-server
COPY ./ ./
RUN cargo build --release
CMD ["./target/release/server"] 

#FROM debian:buster-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
#COPY --from=builder /rust-server/
#CMD ["server"]

