FROM rust:latest

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y g++ && apt-get install build-essential

RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup toolchain install stable-x86_64-unknown-linux-gnu

WORKDIR /app

CMD ["cargo", "build", "--target", "x86_64-unknown-linux-gnu", "--bin", "main", "--release"]

# docker build . -t wav-validator/x86linux -f docker/x86_linux.Dockerfile
# DIR=$(pwd)
# docker run --rm -v $DIR:/app wav-validator/x86linux