all:	dna ui

test: dna-test

update: dna-update ui-update

clean: dna-clean ui-clean

dna:
	(cd dna-src; hc package)

dna-test:
	(cd dna-src; hc test)

dna-start: dna
	(cd dna-src; hc run)

dna-update:
	(cd dna-src/zomes/coolcats/code; cargo update)

dna-clean:
	(cd dna-src/zomes/coolcats/code; cargo clean)
	(cd dna-src/test; rm -rf node_modules package-lock.json)
	find . -name bundle.json -exec rm {} +

ui:
	(cd ui-src; yarn; yarn build)

ui-start:
	(cd ui-src; yarn; yarn start)

ui-update:
	(cd ui-src; cargo update)

ui-clean:
	(cd ui-src; cargo clean)
	(cd ui-src; rm -rf node_modules yarn.lock)
