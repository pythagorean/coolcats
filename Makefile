NIGHTLY = nightly-2019-01-24

all: dna ui

start: dna ui-deploy
	-mkdir tmp-storage
	sed "s;_N3H_;\"`pwd`/../n3h\";" < conductor-config.toml > tmp-storage/conductor-config.toml
	holochain -c tmp-storage/conductor-config.toml

reset:
	rm -rf tmp-storage

fmt: dna-fmt ui-fmt

lint: dna-lint ui-lint

test: dna-test

update: dna-update ui-update
	rustup self update
	rustup update

update-cli:
	rustup default $(NIGHTLY)
	cargo install hc --force --git https://github.com/holochain/holochain-rust.git --branch develop
	rustup default stable

update-conductor:
	rustup default $(NIGHTLY)
	cargo install holochain --force --git https://github.com/holochain/holochain-rust.git --branch develop
	rustup default stable

clean: reset dna-clean ui-clean

build: dna-build ui-build

dna:
	(cd dna-src; hc package)

dna-build: dna

dna-fmt:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) fmt)
	(cd dna-src/test; js-beautify -r -s 2 -n *.js)

dna-lint:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) clippy)

dna-test:
	(cd dna-src/test; yarn)
	(cd dna-src; hc test)

dna-start: dna
	-(cd dna-src; hc run) || make dna-start

dna-update:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) update)

dna-clean:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) clean && rm -f Cargo.lock)
	(cd dna-src/test; rm -rf node_modules package-lock.json)
	find . -name bundle.json -exec rm {} +

ui:
	(cd ui-src; yarn; yarn build)

ui-build: ui

ui-fmt:
	(cd ui-src; cargo +stable fmt)
	(cd ui-src; js-beautify -r -s 2 -n *.js)

ui-lint:
	(cd ui-src; cargo +stable clippy)

ui-start:
	(cd ui-src; yarn; yarn start)

ui-deploy:
	(cd ui-src; yarn; yarn deploy)

ui-update:
	(cd ui-src; cargo +stable update)
	-(cd ui-src; yarn upgrade)

ui-clean:
	(cd ui-src; cargo +stable clean && rm -f Cargo.lock)
	(cd ui-src; rm -rf static node_modules yarn.lock)
