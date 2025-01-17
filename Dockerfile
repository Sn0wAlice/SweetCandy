FROM rust:latest as build

# create a new empty shell project
RUN USER=root cargo new --bin sweetcandy
WORKDIR /sweetcandy

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/sweetcandy*
RUN cargo build --release

# our final base
FROM debian:latest

## install the runtime dependencies
RUN apt-get update

# Install libc
RUN apt-get install -y libc6 ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

# Cleanup
RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# set the timezone
ENV TZ=Europe/Paris

# set the working directory in the image
WORKDIR /sweetcandy

# copy the build artifact from the build stage
COPY --from=build /sweetcandy/target/release/sweetcandy .

# copy the assets, html, utils folder to the final image
COPY ./assets ./assets
COPY ./html ./html
COPY ./utils ./utils
COPY ./vulns ./vulns

# set the startup command to run your binary
CMD ["./sweetcandy", "--prod", "--webhook"]