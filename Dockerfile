# from debian
FROM debian:buster-slim

# install rust
RUN apt-get update && apt-get install -y curl
RUN apt-get install -y build-essential
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly

# copy all codes
COPY . .

# build
RUN cargo build --release

# run
CMD ["./target/release/management"]