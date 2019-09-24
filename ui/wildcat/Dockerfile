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

# Add packages to build and optimize mammoth and presenter
RUN apt-get update && apt-get install -y \
    cmake \
    g++ \
    gcc \
    git \
    musl-tools \
    ninja-build \
    yarn \
  && apt-get clean && rm -rf /var/lib/apt/lists/*

# Drop to user privileges now that system is updated
RUN useradd -ms /bin/bash docker \
  && mkdir -p /home/docker/mammoth \
  && mkdir -p /home/docker/presenter \
  && chown -R docker /home/docker
USER docker

# Build wasm-opt
WORKDIR /home/docker
RUN git clone https://github.com/WebAssembly/binaryen.git
WORKDIR /home/docker/binaryen
RUN cmake -G Ninja . && ninja wasm-opt && mv bin/wasm-opt ~ && ninja clean

# Install Rust + wasm-pack
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y
ENV PATH="/home/docker/.cargo/bin:${PATH}"
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install known good Rust nightly and static target for presenter build
RUN rustup toolchain install nightly-2019-07-14 \
  && rustup default nightly-2019-07-14 \
  && rustup target add x86_64-unknown-linux-musl

# Copy the presenter files
WORKDIR /home/docker/presenter
COPY ./tmp/presenter/Cargo.toml ./
COPY ./tmp/presenter/src/ ./src/

# Build presenter
RUN cargo install --target x86_64-unknown-linux-musl --path . \
  && cargo clean && strip `which presenter`

# Copy the mammoth files
WORKDIR /home/docker/utils
COPY ./tmp/utils/ ./
WORKDIR /home/docker/mammoth
COPY Cargo.toml bootstrap.js package.json tsconfig.json webpack.config.js ./
COPY ./src/ ./src/
COPY ./macros/ ./macros/
COPY ./public/ ./public/

# Install node modules and build mammoth
RUN yarn install && rustup run stable yarn run webpack -p --mode production \
  && cargo clean && rm -rf pkg node_modules yarn.lock

# Optimize the mammoth deployment for size
WORKDIR /home/docker/target/deploy
RUN for file in *.wasm; do \
    ~/wasm-opt -Os -o $file.new $file && mv -f $file.new $file; \
  done; \
  gzip -9 *.wasm *.js fonts/* */*.svg

# Reduce our docker image to the built targets and prepare for docker run
FROM scratch
COPY --from=build /home/docker/.cargo/bin/presenter /presenting
COPY --from=build /home/docker/target/deploy/ /mammoth/
USER 1000
EXPOSE 8000
ENTRYPOINT ["/presenting", "/mammoth"]
