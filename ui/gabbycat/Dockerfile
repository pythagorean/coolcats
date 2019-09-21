FROM ubuntu:bionic

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

# Add packages to build and run gabbycat
RUN apt-get update && apt-get install -y \
    gcc \
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
RUN yarn install && yarn run webpack -p --mode production

# Install known good Rust nightly for presenter build
RUN rustup toolchain install nightly-2019-07-14 && rustup default nightly-2019-07-14

# Copy the presenter files
WORKDIR /home/docker/presenter
COPY ./tmp/presenter/Cargo.toml ./
COPY ./tmp/presenter/src/ ./src/

# Build presenter
RUN cargo install --path . && strip `which presenter`

# Prepared for docker run
WORKDIR /home/docker
EXPOSE 8000
ENTRYPOINT presenter target/deploy
