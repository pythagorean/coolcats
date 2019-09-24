HC_VERSION = 0.0.30-alpha6
RUST_NIGHTLY = nightly-2019-07-14

default: dna ui

fmt: dna-fmt ui-fmt

lint: dna-lint ui-lint

test: dna-test

upgrade:
	git pull
	make rust-upgrade
	make update

rust-upgrade:
	rustup toolchain install $(RUST_NIGHTLY)
	rustup target add wasm32-unknown-unknown --toolchain $(RUST_NIGHTLY)
	rustup component add clippy --toolchain $(RUST_NIGHTLY)

update: dna-update ui-update CARGO-UPDATE-required
	if [ `holochain --version | cut -d ' ' -f 2` != $(HC_VERSION) ]; then make update-conductor; fi
	rustup self update
	rustup update
	cargo install-update -a

update-cli: CARGO-required RUST_NIGHTLY-required
	cargo +$(RUST_NIGHTLY) install hc --force --git https://github.com/holochain/holochain-rust.git --tag v$(HC_VERSION)

update-conductor: CARGO-required RUST_NIGHTLY-required
	cargo +$(RUST_NIGHTLY) install holochain --force --git https://github.com/holochain/holochain-rust.git --tag v$(HC_VERSION)

clean: dna-clean ui-clean vm-clean docker-clean presenter-clean

build: dna-build ui-build

start: conductor-start

stop: conductor-stop

conductor-start: dna ui-deploy HOLOCHAIN_CONDUCTOR-required
	@mkdir -p /tmp/n3h/1
	holochain -c conductor/conductor-config-agent1.toml > /tmp/dna-testnet.log 2>&1 &
	@( tail -f /tmp/dna-testnet.log & ) | grep -q p2p:
	# Disabling multiuser testing until lib3h updates are working
	@#@mkdir -p /tmp/n3h/2
	@#holochain -c conductor/conductor-config-agent2.toml > /tmp/dna-testnet2.log 2>&1 &
	@#@( tail -f /tmp/dna-testnet2.log & ) | grep -q p2p:
	@#@mkdir -p /tmp/n3h/3
	@#holochain -c conductor/conductor-config-agent3.toml > /tmp/dna-testnet3.log 2>&1 &
	@#@( tail -f /tmp/dna-testnet3.log & ) | grep -q p2p:
	@echo Conductor started. Logfiles in /tmp. Run \'make stop\' to stop processes.

conductor-stop:
	killall holochain

dna: dna-build

dna-build: HOLOCHAIN_CLI-required
	rustup run $(RUST_NIGHTLY) hc package

dna-fmt: CARGO-DO-required RUST_NIGHTLY-FMT-required CARGO-TOMLFMT-required JS-BEAUTIFY-required
	for zome in zomes/*; do (cd $$zome/code; cargo +$(RUST_NIGHTLY) do fmt, tomlfmt); done
	for js in test/*.js; do js-beautify -r -s 2 -n $$js || true; done

dna-lint: CARGO-required RUST_NIGHTLY-required
	for zome in zomes/*; do (cd $$zome/code; cargo +$(RUST_NIGHTLY) clippy); done

dna-test: dna-build
	(cd test; yarn -s)
	rustup run $(RUST_NIGHTLY) hc test -s | egrep -v '^[[:blank:]]*(info:|$$)'

dna-start: dna
	hc run || make dna-start

dna-update:
	if [ `hc --version | cut -d ' ' -f 2` != $(HC_VERSION) ]; then make update-cli; fi
	for zome in zomes/*; do (cd $$zome/code; cargo +$(RUST_NIGHTLY) update); done
	#-(cd test; yarn -s; yarn -s upgrade --latest)

dna-clean:
	for zome in zomes/*; do (cd $$zome/code; cargo +$(RUST_NIGHTLY) clean && rm -f Cargo.lock); done
	(cd test; rm -rf node_modules package-lock.json)
	find . -name *.dna.json -exec rm {} +

presenter: presenter-build

presenter-build: CARGO-required RUST_NIGHTLY-required
	(cd presenter; cargo +$(RUST_NIGHTLY) build --release)
	@strip presenter/target/release/presenter

presenter-start: presenter-start-clutter

presenter-start-coolcats: presenter-start-clutter

presenter-start-clutter: presenter ui-deploy-clutter
	@echo "Compressing files to reduce bandwidth"; \
		(cd ui/target/deploy; gzip -9v *.wasm *.js)
	@echo ""
	@echo "Files and file sizes to be served:"
	@wc -c ui/target/deploy/*
	@echo ""
	presenter/target/release/presenter ui/target/deploy &
	@sleep 1
	@echo "Presenter started. Run 'make presenter-stop' to stop process."

presenter-start-mammoth: presenter ui-deploy-mammoth
	@echo "Compressing files to reduce bandwidth:"
	@(cd ui/target/deploy; gzip -9 *.wasm *.js fonts/* */*.svg)
	@wc -c ui/target/deploy/*.gz
	presenter/target/release/presenter ui/target/deploy &
	@sleep 1
	@echo "Presenter started. Run 'make presenter-stop' to stop process."

presenter-stop:
	killall presenter

presenter-stop-coolcats: presenter-stop

presenter-stop-clutter: presenter-stop

presenter-stop-mammoth: presenter-stop

presenter-update: CARGO-required RUST_NIGHTLY-required
	(cd presenter; cargo +$(RUST_NIGHTLY) update)

presenter-clean:
	(cd presenter; cargo +stable clean && rm -f Cargo.lock)
	(cd presenter; rm -rf pkg node_modules yarn.lock)

ui: ui-clutter ui-mammoth

ui-coolcats: ui-clutter

ui-clutter: ui-build-clutter

ui-mammoth: ui-build-mammoth

ui-build: ui-build-clutter ui-build-mammoth

ui-build-coolcats: ui-build-clutter

ui-build-clutter: YARN-required WASM_PACK-required
	(cd ui/clutter; yarn -s; rustup run stable yarn build)

ui-build-mammoth: YARN-required WASM_PACK-required
	(cd ui/mammoth; yarn -s; rustup run stable yarn build)

vm-build-mammoth: VAGRANT-required
	(cd ui/mammoth; vagrant up)

docker-build-mammoth: DOCKER-required
	(cd ui/mammoth; yarn docker-build)

ui-fmt: ui-fmt-clutter ui-fmt-mammoth

ui-fmt-coolcats: ui-fmt-clutter

ui-fmt-clutter: CARGO-DO-required RUST-FMT-required CARGO-TOMLFMT-required JS-BEAUTIFY-required
	(cd ui; cargo +stable do fmt, tomlfmt)
	for js in ui/clutter/*.js; do js-beautify -r -s 2 -n $$js || true; done

ui-fmt-mammoth: CARGO-DO-required RUST-FMT-required CARGO-TOMLFMT-required JS-BEAUTIFY-required
	(cd ui/mammoth; cargo +stable do fmt, tomlfmt)
	for js in ui/mammoth/*.js; do js-beautify -r -s 2 -n $$js || true; done

ui-lint: ui-lint-clutter ui-lint-mammoth

ui-lint-coolcats: ui-lint-clutter

ui-lint-clutter: CARGO-required CLIPPY-required
	(cd ui/clutter; cargo +stable clippy)

ui-lint-mammoth: CARGO-required CLIPPY-required
	(cd ui/mammoth; cargo +stable clippy)

ui-start: ui-start-clutter

ui-start-coolcats: ui-start-clutter

ui-start-clutter: CARGO-required YARN-required WASM_PACK-required
	(cd ui/clutter; yarn -s; rustup run stable yarn start)

ui-start-mammoth: CARGO-required YARN-required WASM_PACK-required
	(cd ui/mammoth; yarn -s; rustup run stable yarn start)

vm-start-mammoth: vm-build-mammoth
	(cd ui/mammoth; vagrant ssh -c "cd /vagrant/mammoth && yarn start" &)
	@sleep 60

docker-start-mammoth: docker-build-mammoth
	(cd ui/mammoth; yarn docker-run)

vm-stop-mammoth: VAGRANT-required
	(cd ui/mammoth; vagrant halt)

docker-stop-mammoth: DOCKER-required
	(cd ui/mammoth; \
		docker stop `docker ps -a -q --filter ancestor=mammoth` || true; \
		docker rm `docker ps -a -q --filter ancestor=mammoth` || true)

vm-clean: vm-clean-mammoth

vm-clean-mammoth:
	-(cd ui/mammoth; vagrant destroy -f && rm -rf .vagrant)

docker-clean: docker-clean-mammoth

docker-clean-mammoth: docker-stop-mammoth
	-(cd ui/mammoth; docker rmi mammoth)

ui-deploy: ui-deploy-clutter

ui-deploy-coolcats: ui-deploy-clutter

ui-deploy-clutter: CARGO-required YARN-required WASM_PACK-required WASM-OPT-recommended
	(cd ui/clutter; yarn -s; rustup run stable yarn deploy)
	make ui-optimize-deployment

ui-deploy-mammoth: CARGO-required YARN-required WASM_PACK-required WASM-OPT-recommended
	(cd ui/mammoth; yarn -s; rustup run stable yarn run webpack -p --mode production)
	make ui-optimize-deployment

ui-optimize-deployment: WASM-OPT-recommended
	@for file in ui/target/deploy/*.wasm; \
		do \
			echo "Optimizing wasm to save space, size shown before and after:"; \
			wc -c $$file; \
			wasm-opt -Os -o $$file.new $$file && mv -f $$file.new $$file; \
			wc -c $$file; \
		done

ui-update: ui-update-clutter ui-update-mammoth

ui-update-clutter: CARGO-required YARN-required
	(cd ui/clutter; cargo +stable update)
	-(cd ui/clutter; yarn -s; yarn -s upgrade --latest)

ui-update-mammoth: CARGO-required YARN-required
	(cd ui/mammoth; cargo +stable update)
	-(cd ui/mammoth; yarn -s; yarn -s upgrade --latest)

ui-clean: ui-clean-clutter ui-clean-mammoth
	(cd ui; cargo +stable clean && rm -f Cargo.lock)

ui-clean-clutter: CARGO-required
	(cd ui/clutter; rm -rf pkg node_modules yarn.lock)

ui-clean-mammoth: CARGO-required
	(cd ui/mammoth; rm -rf pkg node_modules yarn.lock tmp)

YARN-required:
	@which yarn > /dev/null || ( \
		echo "No yarn found. Attempting to install."; \
		curl -o- -L https://yarnpkg.com/install.sh | bash; \
		false; \
 	)

CARGO-required:
	@which cargo > /dev/null || ( \
		echo "No cargo found. Attempting to install Rust."; \
		curl https://sh.rustup.rs -sSf | sh; \
		false; \
	)

CARGO-UPDATE-required:
	@which cargo-install-update > /dev/null || ( \
		echo "Cargo-update not found. Attempting to install."; \
		cargo install cargo-update; \
	)

CARGO-DO-required:
	@which cargo-do > /dev/null || ( \
		echo "Cargo-do not found. Attempting to install."; \
		cargo install cargo-do; \
	)

CARGO-TOMLFMT-required:
	@which cargo-tomlfmt > /dev/null || ( \
		echo "Cargo-tomlfmt not found. Attempting to install."; \
		cargo install cargo-tomlfmt; \
	)

WASM_PACK-required:
	@which wasm-pack > /dev/null || ( \
		echo "No wasm-pack found. Attempting to install."; \
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh; \
	)

RUST_NIGHTLY-required:
	@echo "Checking if required Rust nightly toolchain is installed."
	@rustup toolchain list | grep $(RUST_NIGHTLY) || ( \
		make rust-upgrade; \
	)

RUST-FMT-required:
	@rustup component list --installed --toolchain stable | grep rustfmt || ( \
	  echo "Rustfmt needs to be installed for stable."; \
		rustup component add rustfmt --toolchain stable; \
	)

RUST_NIGHTLY-FMT-required:
	@rustup component list --installed --toolchain $(RUST_NIGHTLY) | grep rustfmt || ( \
	  echo "Rustfmt needs to be installed for nightly."; \
		rustup component add rustfmt --toolchain $(RUST_NIGHTLY); \
	)

JS-BEAUTIFY-required:
	@which js-beautify > /dev/null || ( \
		echo "JS-Beautify not found. Attempting to install."; \
		yarn global add js-beautify; \
	)

CLIPPY-required:
	@rustup component list --installed --toolchain stable | grep clippy || ( \
		echo "Clippy needs to be installed for Rust stable."; \
		rustup component add clippy --toolchain stable; \
	)

HOLOCHAIN_CLI-required:
	@which hc > /dev/null || ( \
	  echo "Holochain CLI needs to be installed."; \
		make update-cli; \
	)

HOLOCHAIN_CONDUCTOR-required:
	@which holochain > /dev/null || ( \
	  echo "Holochain conductor needs to be installed."; \
		make update-conductor; \
	)

WASM-OPT-recommended:
	@which wasm-opt > /dev/null || ( \
		echo "It is recommended to install wasm-opt from binaryen:"; \
		echo "https://github.com/WebAssembly/binaryen"; \
		echo ""; \
	)

VAGRANT-required:
	@which vagrant > /dev/null || ( \
		echo "Vagrant needs to be installed:"; \
		echo "https://www.vagrantup.com/downloads.html"; \
		echo ""; \
		false; \
	)

DOCKER-required:
	@which docker > /dev/null || ( \
		echo "Docker needs to be installed:"; \
		echo "https://www.docker.com/products/docker-desktop"; \
		echo ""; \
		false; \
	)
