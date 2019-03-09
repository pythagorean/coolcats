NIGHTLY = nightly-2019-01-24
N3H = n3h-0.0.4-alpha1

all: dna ui

net-start: dna-startnet ui-startnet

net-stop:
	killall holochain

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
	(cd dna-src/test; yarn)
	(cd dna-src; hc test)

dna-start: dna
	-(cd dna-src; hc run) || make dna-start

dna-startnet: dna
	@if [ ! -d tmp-storage ]; then mkdir tmp-storage; fi
	@sed -e "s;_N3H_;`pwd`/../${N3H};" \
	     -e "s;\"_BOOTSTRAP_\";;" \
       -e "s;_AGENT_;1;g" \
       -e "s;_PORT_;8888;" \
	  < conductor-config.toml > tmp-storage/conductor-config.toml
	holochain -c tmp-storage/conductor-config.toml > tmp-storage/dna-testnet.log 2>&1 &
	@sleep 5
	@cat tmp-storage/dna-testnet.log | grep READY! | sed '/.*\[\"\(.*\)\",.*/ s//\1/' > tmp-storage/dna-testnet.address
	@export BOOTSTRAP=`cat tmp-storage/dna-testnet.address`; \
	  sed -e "s;_N3H_;`pwd`/../${N3H};" \
		    -e "s;_BOOTSTRAP_;$${BOOTSTRAP};" \
				-e "s;_AGENT_;2;g" \
				-e "s;_PORT_;8889;" \
	  < conductor-config.toml > tmp-storage/conductor-config2.toml
	holochain -c tmp-storage/conductor-config2.toml > tmp-storage/dna-testnet2.log 2>&1 &

dna-reset:
	rm -rf tmp-storage

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

ui-startnet: ui-deploy
	http-server ui-src/target/deploy -p8000 -s -c-1 &
	http-server ui-src/target/deploy -p8001 -s -c-1 &
	http-server ui-src/target/deploy -p8002 -s -c-1 &
	fswatch -o ui-src/src | xargs -n 1 -I{} make ui-lint ui-deploy

ui-update:
	(cd ui-src; cargo +stable update)
	-(cd ui-src; yarn upgrade)

ui-clean:
	(cd ui-src; cargo +stable clean && rm -f Cargo.lock)
	(cd ui-src; rm -rf static node_modules yarn.lock)
