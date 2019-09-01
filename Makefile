HC_VERSION = 0.0.29-alpha2
RUST_NIGHTLY = nightly-2019-07-14

all: dna ui

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
	cargo +$(RUST_NIGHTLY) install hc --force --git https://github.com/holochain/holochain-rust.git

update-conductor: CARGO-required RUST_NIGHTLY-required
	cargo +$(RUST_NIGHTLY) install holochain --force --git https://github.com/holochain/holochain-rust.git

clean: dna-clean ui-clean

build: dna-build ui-build

start: conductor-start

stop: conductor-stop

conductor-start: dna ui-deploy HOLOCHAIN_CONDUCTOR-required
	@mkdir -p /tmp/n3h/1
	holochain -c conductor/conductor-config-agent1.toml > /tmp/dna-testnet.log 2>&1 &
	@( tail -f /tmp/dna-testnet.log & ) | grep -q p2p:
	@mkdir -p /tmp/n3h/2
	holochain -c conductor/conductor-config-agent2.toml > /tmp/dna-testnet2.log 2>&1 &
	@( tail -f /tmp/dna-testnet2.log & ) | grep -q p2p:
	@mkdir -p /tmp/n3h/3
	holochain -c conductor/conductor-config-agent3.toml > /tmp/dna-testnet3.log 2>&1 &
	@( tail -f /tmp/dna-testnet3.log & ) | grep -q p2p:
	@echo Conductor started. Logfiles in /tmp. Run \'make stop\' to stop processes.

conductor-stop:
	killall holochain

dna: dna-build

dna-build: HOLOCHAIN_CLI-required
	for f in ui/*.json; do mv $$f $$f.p; done
	rustup run $(RUST_NIGHTLY) hc package
	for f in ui/*.json.p; do mv $$f `echo $$f | cut -f 1,2 -d '.'`; done

dna-fmt: CARGO-DO-required RUST_NIGHTLY-FMT-required CARGO-TOMLFMT-required JS-BEAUTIFY-required
	(cd zomes/coolcats/code; cargo +$(RUST_NIGHTLY) do fmt, tomlfmt)
	for js in test/*.js; do js-beautify -r -s 2 -n $$js || true; done

dna-lint: CARGO-required RUST_NIGHTLY-required
	(cd zomes/coolcats/code; cargo +$(RUST_NIGHTLY) clippy)

dna-test: dna-build
	(cd test; yarn -s)
	rustup run $(RUST_NIGHTLY) hc test -s | egrep -v '^[[:blank:]]*(info:|$$)'

dna-start: dna
	hc run || make dna-start

dna-update:
	if [ `hc --version | cut -d ' ' -f 2` != $(HC_VERSION) ]; then make update-cli; fi
	(cd zomes/coolcats/code; cargo +$(RUST_NIGHTLY) update)
	#-(cd test; yarn -s; yarn -s upgrade --latest)

dna-clean:
	(cd zomes/coolcats/code; cargo +$(RUST_NIGHTLY) clean && rm -f Cargo.lock)
	(cd test; rm -rf node_modules package-lock.json)
	find . -name *.dna.json -exec rm {} +

presenter-start: ui-deploy
	@echo "Compressing files to reduce bandwidth"; \
		(cd ui/target/deploy; gzip -9v *.wasm *.js)
	(cd presenter; cargo +stable build --release)
	@strip presenter/target/release/presenter
	@echo ""
	@echo "Files and file sizes to be served:"
	@wc -c ui/target/deploy/*
	@echo ""
	presenter/target/release/presenter ui/target/deploy &
	@echo "Presenter started. Run 'make presenter-stop' to stop process."
	@sleep 1

presenter-stop:
	killall presenter

ui: ui-build

ui-build: YARN-required WASM_PACK-required
	(cd ui; yarn -s; yarn build)

ui-fmt: CARGO-DO-required RUST-FMT-required CARGO-TOMLFMT-required JS-BEAUTIFY-required
	(cd ui; cargo +stable do fmt, tomlfmt)
	for js in ui/*.js; do js-beautify -r -s 2 -n $$js || true; done

ui-lint: CARGO-required CLIPPY-required
	(cd ui; cargo +stable clippy)

ui-start: YARN-required WASM_PACK-required
	(cd ui; yarn -s; yarn start)

ui-deploy: YARN-required WASM_PACK-required WASM-OPT-recommended
	(cd ui; yarn -s; yarn deploy)
	@for file in ui/target/deploy/*.wasm; \
		do \
			echo "Optimizing wasm to save space, size shown before and after:"; \
			wc -c $$file; \
			wasm-opt -Os -o $$file.new $$file && mv -f $$file.new $$file; \
			wc -c $$file; \
		done

ui-update:
	(cd ui; cargo +stable update)
	-(cd ui; yarn -s; yarn -s upgrade --latest)

ui-clean:
	(cd ui; cargo +stable clean && rm -f Cargo.lock)
	(cd ui; rm -rf pkg node_modules yarn.lock)

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
