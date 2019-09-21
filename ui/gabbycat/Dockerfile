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
  && chown -R docker /home/docker
USER docker

# Install Rust + wasm-pack
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y
ENV PATH="/home/docker/.cargo/bin:${PATH}"
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Copy the local files
WORKDIR /home/docker
COPY ./tmp/utils/ ./utils/
WORKDIR /home/docker/gabbycat
COPY Cargo.toml bootstrap.js package.json tsconfig.json webpack.config.js ./
COPY ./src/ ./src/
COPY ./macros/ ./macros/

# Install node modules
RUN yarn install

# Build gabbycat
RUN yarn run webpack -p --mode production

# Prepared for docker run
EXPOSE 8000
ENTRYPOINT yarn run webpack-dev-server -p --mode production
