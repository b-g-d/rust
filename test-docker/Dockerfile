############################
## Builder
############################
FROM rust:1.59 as builder

# create empty shell project
RUN USER=root cargo new --bin test-docker
WORKDIR ./test-docker
COPY ./Cargo.* ./
RUN cargo build --release
RUN rm src/*.rs

COPY . ./
RUN rm ./target/release/deps/*
RUN cargo build --release

############################
## Final release
############################
	
FROM debian:buster-slim

WORKDIR /test-docker

COPY --from=builder /test-docker/target/release/test-docker ./
CMD ["./test-docker"]