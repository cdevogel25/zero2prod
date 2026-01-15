# use latest stable rust
FROM rust:1.92.0

# change working directory to `app`
# will be created by Docker if it doesn't already exist
WORKDIR /app

# install the required system deps for linking
RUN apt update && apt install lld clang -y

# copy all the files from our environment to the docker image
COPY . .

# tell sqlx to look at offline queries
ENV SQLX_OFFLINE=true

# build the binary! (with --release for speed)
RUN cargo build --release

# set environment
ENV APP_ENVIRONMENT=production

# when docker run, launch the built binary
ENTRYPOINT ["./target/release/zero2prod"]