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
# -m: create a home directory.
# -U: create a same name of user group.
RUN useradd -U -m svg
USER svg
RUN mkdir /home/svg/app
WORKDIR /home/svg/app
COPY --from=builder --chown=svg:svg /build/react-ui/build /home/svg/app/react-ui/build
COPY --from=builder --chown=svg:svg /build/target/release/svg-web-service /home/svg/app
COPY --from=builder --chown=svg:svg /build/docker-env /home/svg/app/.env
EXPOSE 8080 
CMD ["./svg-web-service"]