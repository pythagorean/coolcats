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

present: present-coolcats

present-coolcats: present-housecat

present-housecat: presenter-start dna-start presenter-stop

present-wildcat: presenter-start-wildcat dna-start presenter-stop

presenter: presenter-build

presenter-build: CARGO-required RUST_NIGHTLY-required
	(cd presenter; cargo +$(RUST_NIGHTLY) build --release)
	@strip presenter/target/release/presenter

presenter-start: presenter-start-coolcats

presenter-start-coolcats: presenter-start-housecat

presenter-start-housecat: presenter ui-deploy-housecat
	@echo "Compressing files to reduce bandwidth"; \
		(cd ui/target/deploy; gzip -9v *.wasm *.js)
	@echo ""
	@echo "Files and file sizes to be served:"
	@wc -c ui/target/deploy/*
	@echo ""
	presenter/target/release/presenter ui/target/deploy &
	@sleep 1
	@echo "Presenter started. Run 'make presenter-stop' to stop process."

presenter-start-wildcat: presenter ui-deploy-wildcat
	@echo "Compressing files to reduce bandwidth:"
	@(cd ui/target/deploy; gzip -9 *.wasm *.js fonts/* */*.svg)
	@wc -c ui/target/deploy/*.gz
	presenter/target/release/presenter ui/target/deploy &
	@sleep 1
	@echo "Presenter started. Run 'make presenter-stop' to stop process."

presenter-stop:
	killall presenter

presenter-stop-coolcats: presenter-stop

presenter-stop-housecat: presenter-stop

presenter-stop-wildcat: presenter-stop

presenter-restart-coolcats: presenter-stop presenter-start-coolcats

presenter-restart-housecat: presenter-stop presenter-start-housecat

presenter-restart-wildcat: presenter-stop presenter-start-wildcat

presenter-update: CARGO-required RUST_NIGHTLY-required
	(cd presenter; cargo +$(RUST_NIGHTLY) update)

presenter-clean:
	(cd presenter; cargo +stable clean && rm -f Cargo.lock)
	(cd presenter; rm -rf pkg node_modules yarn.lock)

ui: ui-housecat ui-wildcat

ui-coolcats: ui-housecat

ui-housecat: ui-build-housecat

ui-wildcat: ui-build-wildcat

ui-build: ui-build-housecat ui-build-wildcat

ui-build-coolcats: ui-build-housecat

ui-build-housecat: YARN-required WASM_PACK-required
	(cd ui/housecat; yarn -s; rustup run stable yarn build)

ui-build-wildcat: YARN-required WASM_PACK-required
	(cd ui/wildcat; yarn -s; rustup run stable yarn build)

vm-build-wildcat: VAGRANT-required
	(cd ui/wildcat; vagrant up)

docker-build-wildcat: DOCKER-required
	(cd ui/wildcat; yarn docker-build)

ui-fmt: ui-fmt-housecat ui-fmt-wildcat

ui-fmt-coolcats: ui-fmt-housecat

ui-fmt-housecat: CARGO-DO-required RUST-FMT-required CARGO-TOMLFMT-required JS-BEAUTIFY-required
	(cd ui; cargo +stable do fmt, tomlfmt)
	for js in ui/housecat/*.js; do js-beautify -r -s 2 -n $$js || true; done

ui-fmt-wildcat: CARGO-DO-required RUST-FMT-required CARGO-TOMLFMT-required JS-BEAUTIFY-required
	(cd ui/wildcat; cargo +stable do fmt, tomlfmt)
	for js in ui/wildcat/*.js; do js-beautify -r -s 2 -n $$js || true; done

ui-lint: ui-lint-housecat ui-lint-wildcat

ui-lint-coolcats: ui-lint-housecat

ui-lint-housecat: CARGO-required CLIPPY-required
	(cd ui/housecat; cargo +stable clippy)

ui-lint-wildcat: CARGO-required CLIPPY-required
	(cd ui/wildcat; cargo +stable clippy)

ui-start: ui-start-housecat

ui-start-coolcats: ui-start-housecat

ui-start-housecat: CARGO-required YARN-required WASM_PACK-required
	(cd ui/housecat; yarn -s; rustup run stable yarn start)

ui-start-wildcat: CARGO-required YARN-required WASM_PACK-required
	(cd ui/wildcat; yarn -s; rustup run stable yarn start)

vm-start-wildcat: vm-build-wildcat
	(cd ui/wildcat; vagrant ssh -c "cd /vagrant/wildcat && yarn start" &)
	@sleep 60

docker-start-wildcat: docker-build-wildcat
	(cd ui/wildcat; yarn docker-run)

vm-stop-wildcat: VAGRANT-required
	(cd ui/wildcat; vagrant halt)

docker-stop-wildcat: DOCKER-required
	(cd ui/wildcat; \
		docker stop `docker ps -a -q --filter ancestor=wildcat` || true; \
		docker rm `docker ps -a -q --filter ancestor=wildcat` || true)

vm-clean: vm-clean-wildcat

vm-clean-wildcat:
	-(cd ui/wildcat; vagrant destroy -f && rm -rf .vagrant)

docker-clean: docker-clean-wildcat

docker-clean-wildcat: docker-stop-wildcat
	-(cd ui/wildcat; docker rmi wildcat)

ui-deploy: ui-deploy-housecat

ui-deploy-coolcats: ui-deploy-housecat

ui-deploy-housecat: CARGO-required YARN-required WASM_PACK-required WASM-OPT-recommended
	(cd ui/housecat; yarn -s; rustup run stable yarn deploy)
	make ui-optimize-deployment

ui-deploy-wildcat: CARGO-required YARN-required WASM_PACK-required WASM-OPT-recommended
	(cd ui/wildcat; yarn -s; rustup run stable yarn run webpack -p --mode production)
	make ui-optimize-deployment

ui-optimize-deployment: WASM-OPT-recommended
	@for file in ui/target/deploy/*.wasm; \
		do \
			echo "Optimizing wasm to save space, size shown before and after:"; \
			wc -c $$file; \
			wasm-opt -Os -o $$file.new $$file && mv -f $$file.new $$file; \
			wc -c $$file; \
		done

ui-update: ui-update-housecat ui-update-wildcat

ui-update-housecat: CARGO-required YARN-required
	(cd ui/housecat; cargo +stable update)
	-(cd ui/housecat; yarn -s; yarn -s upgrade --latest)

ui-update-wildcat: CARGO-required YARN-required
	(cd ui/wildcat; cargo +stable update)
	-(cd ui/wildcat; yarn -s; yarn -s upgrade --latest)

ui-clean: ui-clean-housecat ui-clean-wildcat
	(cd ui; cargo +stable clean && rm -f Cargo.lock)

ui-clean-housecat: CARGO-required
	(cd ui/housecat; rm -rf pkg node_modules yarn.lock)

ui-clean-wildcat: CARGO-required
	(cd ui/wildcat; rm -rf pkg node_modules yarn.lock tmp)

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
