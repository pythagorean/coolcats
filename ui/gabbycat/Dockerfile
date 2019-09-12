FROM ubuntu:bionic
EXPOSE 8000

# Update our package database and add installation prerequisites
RUN apt-get update && apt-get install -y \
    curl \
    gnupg \
    software-properties-common \
  && apt-get clean

# Add the yarn repo + yarn repo keys
RUN curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN apt-add-repository 'deb https://dl.yarnpkg.com/debian/ stable main'

# Add repo for NodeJS
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash -

# Add packages to build and run gabbycat
RUN apt-get install -y \
    gcc \
    yarn \
  && apt-get clean

# Drop to user privileges now that system is updated
RUN useradd -ms /bin/bash docker
RUN mkdir -p /home/docker/gabbycat
RUN chown -R docker /home/docker
WORKDIR /home/docker/gabbycat
USER docker

# Install Rust + wasm-pack
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh /dev/stdin -y
ENV PATH="/home/docker/.cargo/bin:${PATH}"
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Copy the local files
COPY Cargo.toml bootstrap.js package.json tsconfig.json webpack.config.js ./
COPY ./src/ ./src/

# Install node modules
RUN yarn install

# Build gabbycat
RUN yarn build

ENTRYPOINT yarn start
RUN echo "To start server:" && echo "  $ docker run -p 8000:8000 -P gabbycat"
