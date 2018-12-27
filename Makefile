all:	dna ui

test: dna-test

update: dna-update ui-update
	rustup update

clean: dna-clean ui-clean

build: dna-build ui-build

dna:
	(cd dna-src; hc package)

dna-build: dna

dna-test:
	(cd dna-src; hc test)

dna-start: dna
	(cd dna-src; hc run)

dna-update:
	(cd dna-src/zomes/coolcats/code; cargo +nightly update)

dna-clean:
	(cd dna-src/zomes/coolcats/code; cargo +nightly clean)
	(cd dna-src/test; rm -rf node_modules package-lock.json)
	find . -name bundle.json -exec rm {} +

ui:
	(cd ui-src; yarn; yarn build)

ui-build: ui

ui-start:
	(cd ui-src; yarn; yarn start)

ui-update:
	(cd ui-src; cargo +stable update)

ui-clean:
	(cd ui-src; cargo +stable clean)
	(cd ui-src; rm -rf node_modules yarn.lock)
