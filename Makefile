HC_VERSION = 0.0.28-alpha1
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

update: dna-update ui-update
	if [ `holochain --version | cut -d ' ' -f 2` != $(HC_VERSION) ]; then make update-conductor; fi
	rustup self update
	rustup update
	cargo install-update -a

update-cli:
	cargo +$(RUST_NIGHTLY) install hc --force --git https://github.com/holochain/holochain-rust.git

update-conductor:
	cargo +$(RUST_NIGHTLY) install holochain --force --git https://github.com/holochain/holochain-rust.git

clean: dna-clean ui-clean

build: dna-build ui-build

start: conductor-start

stop: conductor-stop

conductor-start: dna ui-deploy
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

dna-build:
	for f in ui/*.json; do mv $$f $$f.p; done
	rustup run $(RUST_NIGHTLY) hc package
	for f in ui/*.json.p; do mv $$f `echo $$f | cut -f 1,2 -d '.'`; done

dna-fmt:
	(cd zomes/coolcats/code; cargo +$(RUST_NIGHTLY) do fmt, tomlfmt)
	(cd test; js-beautify -r -s 2 -n *.js)

dna-lint:
	(cd zomes/coolcats/code; cargo +$(RUST_NIGHTLY) clippy)

dna-test: dna-build
	(cd test; yarn -s)
	rustup run $(RUST_NIGHTLY) hc test -s | egrep -v '^[[:blank:]]*(info:|$$)'

dna-start: dna
	hc run || make dna-start

dna-update:
	if [ `hc --version | cut -d ' ' -f 2` != $(HC_VERSION) ]; then make update-cli; fi
	(cd zomes/coolcats/code; cargo +$(RUST_NIGHTLY) update)
	-(cd test; yarn -s; yarn -s upgrade --latest)

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
	presenter/target/release/presenter ui/target/deploy

ui: ui-build

ui-build:
	(cd ui; yarn -s; yarn build)

ui-fmt:
	(cd ui; cargo +stable do fmt, tomlfmt)
	(cd ui; js-beautify -r -s 2 -n *.js)

ui-lint:
	(cd ui; cargo +stable clippy)

ui-start:
	(cd ui; yarn -s; yarn start)

ui-deploy:
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
	(cd ui; rm -rf static node_modules yarn.lock)
