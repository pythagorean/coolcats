NIGHTLY = nightly-2019-01-24

all:	dna ui

fmt: dna-fmt ui-fmt

lint: dna-lint ui-lint

test: dna-test

update: dna-update ui-update
	rustup self update
	rustup update

clean: dna-clean ui-clean

build: dna-build ui-build

dna:
	(cd dna-src; hc package)

dna-build: dna

dna-fmt:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) fmt)

dna-lint:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) clippy)

dna-test:
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

ui-lint:
	(cd ui-src; cargo +stable clippy)

ui-start:
	(cd ui-src; yarn; yarn start)

ui-update:
	(cd ui-src; cargo +stable update)
	(cd ui-src; yarn upgrade)

ui-clean:
	(cd ui-src; cargo +stable clean && rm -f Cargo.lock)
	(cd ui-src; rm -rf static node_modules yarn.lock)
