FROM rust:1.68.2 as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY packages ./packages


ARG packageNames=wallet,amm


WORKDIR /app/packages
## TODO: write the script to build each of the packages individually.


