# builder image to build the project and generate the binaries
FROM rust:1.47.0-slim-buster as builder

RUN apt-get update
RUN apt-get --assume-yes install default-libmysqlclient-dev

COPY . /source

WORKDIR /source

RUN rustup default nightly
RUN cargo build --release

# final image
FROM rust:1.47.0-slim-buster

RUN adduser --disabled-password --gecos '' app-user

RUN apt-get update
RUN apt-get --assume-yes install default-libmysqlclient-dev

COPY --from=builder --chown=app-user /source/target/release /home/app-user/release
COPY --chown=app-user ./migrations/ /home/app-user/migrations
COPY --chown=app-user ./scripts/ /home/app-user

RUN chmod +x /home/app-user/*.sh
RUN cargo install diesel_cli --no-default-features --features mysql

EXPOSE 8000
USER app-user
WORKDIR /home/app-user

ENTRYPOINT ["./entrypoint.sh"]
