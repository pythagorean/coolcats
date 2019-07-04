HC_VERSION = 0.0.22-alpha1
RUST_NIGHTLY = nightly-2019-01-24

all: dna ui

fmt: dna-fmt ui-fmt

lint: dna-lint ui-lint

test: dna-test

upgrade:
	git pull
	make update

update: dna-update ui-update
	if [ `holochain --version | cut -d ' ' -f 2` != $(HC_VERSION) ]; then make update-conductor; fi
	rustup self update
	rustup update
	cargo install-update -a

update-cli:
	cargo +$(RUST_NIGHTLY) install hc --force --git https://github.com/holochain/holochain-rust.git --tag v$(HC_VERSION)

update-conductor:
	cargo +$(RUST_NIGHTLY) install holochain --force --git https://github.com/holochain/holochain-rust.git --tag v$(HC_VERSION)

clean: dna-clean ui-clean

build: dna-build ui-build

start: conductor-start

stop: conductor-stop

conductor-start: dna ui-deploy
	@mkdir -p /tmp/n3h/1
	holochain -c conductor/conductor-config-agent1.toml > /tmp/dna-testnet.log 2>&1 &
	@( tail +1 -f /tmp/dna-testnet.log & ) | grep -q p2p:
	@mkdir -p /tmp/n3h/2
	holochain -c conductor/conductor-config-agent2.toml > /tmp/dna-testnet2.log 2>&1 &
	@( tail +1 -f /tmp/dna-testnet2.log & ) | grep -q p2p:
	@mkdir -p /tmp/n3h/3
	holochain -c conductor/conductor-config-agent3.toml > /tmp/dna-testnet3.log 2>&1 &
	@( tail +1 -f /tmp/dna-testnet3.log & ) | grep -q p2p:
	@echo Conductor started. Logfiles in /tmp. Run \'make stop\' to stop processes.

conductor-stop:
	killall holochain
	pkill -f n3h.app

dna: dna-build

dna-build:
	(cd dna-src; mkdir dist; rustup run $(RUST_NIGHTLY) hc package -o dist/coolcats.dna.json)
	-ln -s coolcats.dna.json dna-src/dist/dna-src.dna.json

dna-fmt:
	(cd dna-src/zomes/coolcats/code; cargo +$(RUST_NIGHTLY) do fmt, tomlfmt)
	(cd dna-src/test; js-beautify -r -s 2 -n *.js)

dna-lint:
	(cd dna-src/zomes/coolcats/code; cargo +$(RUST_NIGHTLY) clippy)

dna-test: dna-build
	(cd dna-src/test; yarn -s)
	(cd dna-src; rustup run $(RUST_NIGHTLY) hc test -s) | egrep -v '^[[:blank:]]*(info:|$$)'

dna-start: dna
	-(cd dna-src; hc run) || make dna-start

dna-update:
	if [ `hc --version | cut -d ' ' -f 2` != $(HC_VERSION) ]; then make update-cli; fi
	(cd dna-src/zomes/coolcats/code; cargo +$(RUST_NIGHTLY) update)
	-(cd dna-src/test; yarn -s upgrade --latest)

dna-clean:
	(cd dna-src/zomes/coolcats/code; cargo +$(RUST_NIGHTLY) clean && rm -f Cargo.lock)
	(cd dna-src/test; rm -rf node_modules package-lock.json)
	find . -name *.dna.json -exec rm {} +

ui: ui-build

ui-build:
	(cd ui-src; yarn -s; yarn build)

ui-fmt:
	(cd ui-src; cargo +stable do fmt, tomlfmt)
	(cd ui-src; js-beautify -r -s 2 -n *.js)

ui-lint:
	(cd ui-src; cargo +stable clippy)

ui-start:
	(cd ui-src; yarn -s; yarn start)

ui-deploy:
	(cd ui-src; yarn -s; yarn deploy)

ui-update:
	(cd ui-src; cargo +stable update)
	-(cd ui-src; yarn -s upgrade --latest)

ui-clean:
	(cd ui-src; cargo +stable clean && rm -f Cargo.lock)
	(cd ui-src; rm -rf static node_modules yarn.lock)
