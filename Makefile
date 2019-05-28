NIGHTLY = nightly-2019-01-24
VERSION = --tag v0.0.17-alpha2

all: dna ui

fmt: dna-fmt ui-fmt

lint: dna-lint ui-lint

test: dna-test

update: dna-update ui-update
	rustup self update
	rustup update
	cargo install-update -a

update-cli:
	cargo +$(NIGHTLY) install hc --force --git https://github.com/holochain/holochain-rust.git $(VERSION)

update-conductor:
	cargo +$(NIGHTLY) install holochain --force --git https://github.com/holochain/holochain-rust.git $(VERSION)

clean: dna-clean ui-clean

build: dna-build ui-build

conductor-start: dna ui-deploy
	holochain -c conductor/conductor-config-agent1.toml

dna: dna-build

dna-build:
	(cd dna-src; mkdir dist; rustup run $(NIGHTLY) hc package -o dist/coolcats.dna.json)
	-ln -s coolcats.dna.json dna-src/dist/dna-src.dna.json

dna-fmt:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) do fmt, tomlfmt)
	(cd dna-src/test; js-beautify -r -s 2 -n *.js)

dna-lint:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) clippy)

dna-test: dna-build
	(cd dna-src/test; yarn -s)
	(cd dna-src; rustup run $(NIGHTLY) hc test -s)

dna-start: dna
	-(cd dna-src; hc run) || make dna-start

dna-update:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) update)
	-(cd dna-src/test; yarn -s upgrade --latest)

dna-clean:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) clean && rm -f Cargo.lock)
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
