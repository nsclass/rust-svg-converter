# Copyright (c) 2020 Nam Seob Seo
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

# build stage
FROM node:12-stretch as builder

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
RUN export PATH=$HOME/.cargo/bin:$PATH
RUN apt-get update && apt-get upgrade -y && apt-get install -y kcov && apt-get install -y yarn
RUN rm -rf /var/lib/apt/lists/*
WORKDIR /build
COPY . .

# build react-ui application
# RUN npm i -g yarn
WORKDIR /build/react-ui
RUN yarn install
RUN yarn build

# compile rust application
WORKDIR /build
RUN $HOME/.cargo/bin/cargo build --release

# runtim stage
FROM ubuntu
WORKDIR /app
COPY --from=builder /build/react-ui/build /app/react-ui/build
COPY --from=builder /build/target/release/svg-web-service /app
COPY --from=builder /build/docker-env /app/.env
EXPOSE 8080 
CMD ["./svg-web-service"]