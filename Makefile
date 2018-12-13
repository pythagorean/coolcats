all:	dna ui

test: dna-test

update: dna-update ui-update

clean: dna-clean ui-clean

dna:
	(cd dna-src; hc package)

dna-test:
	(cd dna-src; hc test)

dna-update:
	(cd dna-src/zomes/coolcats/code; cargo update)

dna-clean:
	(cd dna-src/zomes/coolcats/code; cargo clean)
	(cd dna-src/test; rm -rf node_modules package-lock.json)
	find . -name bundle.json -exec rm {} +

ui:
	(cd ui-src; cargo web build --release)

ui-start:
	(cd ui-src; cargo web start --release)

ui-update:
	(cd ui-src; cargo update)

ui-clean:
	(cd ui-src; cargo clean)