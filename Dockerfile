# Copyright (c) 2020 Nam Seob Seo
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

# build stage
FROM node:22-bookworm AS builder

ENV PATH=/root/.cargo/bin:$PATH

RUN apt-get update \
    && apt-get install -y --no-install-recommends curl ca-certificates build-essential pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y \
    && npm install -g yarn

WORKDIR /build
COPY . .

# build react-ui application
WORKDIR /build/react-ui
RUN yarn install --frozen-lockfile
RUN yarn build

# compile rust application
WORKDIR /build
RUN cargo build --release

# runtime stage
FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -U -m svg

USER svg
RUN mkdir /home/svg/app
WORKDIR /home/svg/app
COPY --from=builder --chown=svg:svg /build/react-ui/build /home/svg/app/react-ui/build
COPY --from=builder --chown=svg:svg /build/target/release/svg-web-service /home/svg/app
COPY --from=builder --chown=svg:svg /build/docker-env /home/svg/app/.env
EXPOSE 8080
CMD ["./svg-web-service"]