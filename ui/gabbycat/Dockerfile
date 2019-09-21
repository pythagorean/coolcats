FROM ubuntu:bionic AS build

# Add installation prerequisites
RUN apt-get update && apt-get install -y \
    curl \
    gnupg \
    software-properties-common \
  && apt-get clean && rm -rf /var/lib/apt/lists/*

# Add the yarn repo + yarn repo keys
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN apt-add-repository 'deb https://dl.yarnpkg.com/debian/ stable main'

# Add repo for NodeJS
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash -

# Add packages to build gabbycat and presenter
RUN apt-get update && apt-get install -y \
    gcc \
    musl-tools \
    yarn \
  && apt-get clean && rm -rf /var/lib/apt/lists/*

# Drop to user privileges now that system is updated
RUN useradd -ms /bin/bash docker \
  && mkdir -p /home/docker/gabbycat \
  && mkdir -p /home/docker/presenter \
  && chown -R docker /home/docker
USER docker

# Install Rust + wasm-pack
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y
ENV PATH="/home/docker/.cargo/bin:${PATH}"
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Copy the gabbycat files
WORKDIR /home/docker/utils
COPY ./tmp/utils/ ./
WORKDIR /home/docker/gabbycat
COPY Cargo.toml bootstrap.js package.json tsconfig.json webpack.config.js ./
COPY ./src/ ./src/
COPY ./macros/ ./macros/

# Install node modules and build gabbycat
RUN yarn install && yarn run webpack -p --mode production \
  && cargo clean && rm -rf pkg node_modules yarn.lock

# Install known good Rust nightly and static target for presenter build
RUN rustup toolchain install nightly-2019-07-14 && rustup default nightly-2019-07-14
RUN rustup target add x86_64-unknown-linux-musl

# Copy the presenter files
WORKDIR /home/docker/presenter
COPY ./tmp/presenter/Cargo.toml ./
COPY ./tmp/presenter/src/ ./src/

# Build presenter
RUN cargo install --target x86_64-unknown-linux-musl --path . && cargo clean

# Reduce our docker image to the built targets and prepare for docker run
FROM alpine:3.9
COPY --from=build /home/docker/target/deploy/ ./deploy/
COPY --from=build /home/docker/.cargo/bin/presenter ./
USER 1000
EXPOSE 8000
ENTRYPOINT ./presenter deploy
