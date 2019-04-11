NIGHTLY = nightly-2019-01-24
VERSION = --tag v0.0.11-alpha1
N3H = n3h-0.0.9-alpha2

all: dna ui

startnet: dna-startnet ui-startnet

stopnet: dna-stopnet

fmt: dna-fmt ui-fmt

lint: dna-lint ui-lint

test: dna-test

update: dna-update ui-update
	rustup self update
	rustup update

update-cli:
	rustup default $(NIGHTLY)
	cargo install hc --force --git https://github.com/holochain/holochain-rust.git $(VERSION)
	rustup default stable

update-conductor:
	rustup default $(NIGHTLY)
	cargo install holochain --force --git https://github.com/holochain/holochain-rust.git $(VERSION)
	rustup default stable

clean: reset dna-clean ui-clean

reset: dna-reset

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
	(cd dna-src/test; yarn -s)
	(cd dna-src; hc test)

dna-start: dna
	-(cd dna-src; hc run) || make dna-start

dna-startnet: dna
	@if [ ! -d tmp-storage ]; then mkdir tmp-storage; fi
	@sed -e "s;_N3H_;`pwd`/../${N3H};" \
	  < conductor/conductor-config.tmpl > tmp-storage/conductor-config.toml
	holochain -c tmp-storage/conductor-config.toml > tmp-storage/dna-testnet.log 2>&1 &
	@sleep 30
	@cat tmp-storage/dna-testnet.log | grep p2p: | cut -d'"' -f 2 > tmp-storage/dna-testnet.address
	@export BOOTSTRAP=`cat tmp-storage/dna-testnet.address`; \
	  sed -e "s;_N3H_;`pwd`/../${N3H};" \
		    -e "s;_BOOTSTRAP_;$${BOOTSTRAP};" \
	  < conductor/conductor-config2.tmpl > tmp-storage/conductor-config2.toml
	holochain -c tmp-storage/conductor-config2.toml > tmp-storage/dna-testnet2.log 2>&1 &
	@export BOOTSTRAP=`cat tmp-storage/dna-testnet.address`; \
	  sed -e "s;_N3H_;`pwd`/../${N3H};" \
		    -e "s;_BOOTSTRAP_;$${BOOTSTRAP};" \
	  < conductor/conductor-config3.tmpl > tmp-storage/conductor-config3.toml
	# performance issues currently if 3 holochains started
	#holochain -c tmp-storage/conductor-config3.toml > tmp-storage/dna-testnet3.log 2>&1 &

dna-stopnet:
	killall holochain
	killall node

dna-reset:
	rm -rf tmp-storage

dna-update:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) update)
	-(cd dna-src/test; yarn -s upgrade --latest)

dna-clean:
	(cd dna-src/zomes/coolcats/code; cargo +$(NIGHTLY) clean && rm -f Cargo.lock)
	(cd dna-src/test; rm -rf node_modules package-lock.json)
	find . -name *.dna.json -exec rm {} +

ui:
	(cd ui-src; yarn -s; yarn build)

ui-build: ui

ui-fmt:
	(cd ui-src; cargo +stable fmt)
	(cd ui-src; js-beautify -r -s 2 -n *.js)

ui-lint:
	(cd ui-src; cargo +stable clippy)

ui-start:
	(cd ui-src; yarn -s; yarn start)

ui-deploy:
	(cd ui-src; yarn -s; yarn deploy)

ui-startnet: ui-deploy
	http-server ui-src/target/deploy -p8000 -s -g -c-1 &
	http-server ui-src/target/deploy -p8001 -s -g -c-1 &
	http-server ui-src/target/deploy -p8002 -s -g -c-1 &
	fswatch -o ui-src/src | xargs -n 1 -I{} make ui-lint ui-deploy

ui-update:
	(cd ui-src; cargo +stable update)
	-(cd ui-src; yarn -s upgrade --latest)

ui-clean:
	(cd ui-src; cargo +stable clean && rm -f Cargo.lock)
	(cd ui-src; rm -rf static node_modules yarn.lock)
