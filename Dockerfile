# select image
FROM magnonellie/amethyst-dependencies:latest

# create a new empty shell project
RUN USER=root cargo new --bin purple
WORKDIR /purple

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for debug
RUN rm ./target/debug/deps/purple*
RUN cargo build

# set the startup command to run your binary
CMD ["./target/debug/purple"]