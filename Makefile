all:	dna ui

test: dna-test

dna:
	(cd dna-src; hc package)

dna-test:
	(cd dna-src; hc test)

ui:
	(cd ui-src; cargo web build)

ui-start:
	(cd ui-src; cargo web start)
