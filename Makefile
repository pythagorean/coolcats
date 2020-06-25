unexport SSL_CERT_FILE
export PATH := node_modules/.bin:$(PATH)

all: dna ui

fmt: dna-fmt ui-fmt

lint: dna-lint ui-lint

test: dna-test

clean: dna-clean ui-clean presenter-clean
	rm -rf .cargo dist node_modules yarn.lock

build: dna-build ui-build

start: conductor-start

stop: conductor-stop

conductor-start: dna ui-deploy
	@mkdir -p /tmp/n3h/1
	holochain -c conductor/conductor-config-agent1.toml > /tmp/dna-testnet.log 2>&1 &
	@( tail -f /tmp/dna-testnet.log & ) | grep -q p2p:
	# Disabling multiuser testing until lib3h updates are working
	@#@mkdir -p /tmp/n3h/2
	@#holochain -c conductor/conductor-config-agent2.toml > /tmp/dna-testnet2.log 2>&1 &
	@#@( tail -f /tmp/dna-testnet2.log & ) | grep -q p2p:
	@#@mkdir -p /tmp/n3h/3
	@#holochain -c conductor/conductor-config-agent3.toml > /tmp/dna-testnet3.log 2>&1 &
	@#@( tail -f /tmp/dna-testnet3.log & ) | grep -q p2p:
	@echo Conductor started. Logfiles in /tmp. Run \'make stop\' to stop processes.

conductor-stop:
	killall holochain

dna: dna-build

dna-build:
	hc package

dna-fmt: CARGO-DO-required CARGO-TOMLFMT-required JS-BEAUTIFY-required
	for zome in zomes/*; do (cd $$zome/code; cargo do fmt, tomlfmt); done
	for js in test/*.js; do js-beautify -r -s 2 -n $$js || true; done

dna-lint:
	for zome in zomes/*; do (cd $$zome/code; cargo clippy); done

dna-test: dna-build
	(cd test; yarn -s)
	hc test -s | egrep -v '^[[:blank:]]*(info:|$$)'

dna-start: dna
	hc run || make dna-start

dna-update:
	for zome in zomes/*; do (cd $$zome/code; cargo update); done
	#-(cd test; yarn -s; yarn -s upgrade --latest)

dna-clean:
	for zome in zomes/*; do (cd $$zome/code; cargo clean && rm -f Cargo.lock); done
	(cd test; rm -rf node_modules package-lock.json yarn.lock)
	find . -name *.dna.json -exec rm {} +

presenter-start: presenter-start-standard

presenter-start-standard: ui-deploy-standard
	@echo "Compressing files to reduce bandwidth"; \
		(cd ui/target/deploy; gzip -9v *.wasm *.js)
	(cd presenter; cargo build --release)
	@strip target/release/presenter
	@echo ""
	@echo "Files and file sizes to be served:"
	@wc -c ui/target/deploy/*
	@echo ""
	target/release/presenter ui/target/deploy &
	@echo "Presenter started. Run 'make presenter-stop' to stop process."
	@sleep 1

presenter-stop:
	killall presenter

presenter-stop-standard: presenter-stop

presenter-clean:
	(cd presenter; cargo clean && rm -f Cargo.lock)
	(cd presenter; rm -rf pkg node_modules yarn.lock)

ui: ui-standard

ui-standard: ui-build-standard

ui-build: ui-build-standard

ui-build-standard:
	(cd ui/standard; yarn -s; yarn build)

ui-fmt: CARGO-DO-required RUST-FMT-required CARGO-TOMLFMT-required JS-BEAUTIFY-required
	for ui in ui/*; do (cd $$ui; cargo do fmt, tomlfmt); done
	for js in ui/*/*.js; do js-beautify -r -s 2 -n $$js || true; done

ui-lint: ui-lint-standard

ui-lint-standard:
	(cd ui/standard; cargo clippy)

ui-start: ui-start-standard

ui-start-standard:
	(cd ui/standard; yarn -s; yarn start)

ui-deploy: ui-deploy-standard

ui-deploy-standard:
	(cd ui/standard; yarn -s; yarn deploy)
	@for file in ui/target/deploy/*.wasm; \
		do \
			echo "Optimizing wasm to save space, size shown before and after:"; \
			wc -c $$file; \
			wasm-opt -Os -o $$file.new $$file && mv -f $$file.new $$file; \
			wc -c $$file; \
		done

ui-update: ui-update-standard

ui-update-standard: YARN-required
	(cd ui/standard; cargo update)
	-(cd ui/standard; yarn -s; yarn -s upgrade --latest)

ui-clean: ui-clean-standard

ui-clean-standard:
	(cd ui; cargo clean && rm -f Cargo.lock)
	(cd ui/standard; rm -rf pkg node_modules yarn.lock)

CARGO-DO-required:
	@which cargo-do > /dev/null || ( \
		echo "Cargo-do not found. Attempting to install."; \
		cargo install cargo-do; \
	)

CARGO-TOMLFMT-required:
	@which cargo-tomlfmt > /dev/null || ( \
		echo "Cargo-tomlfmt not found. Attempting to install."; \
		cargo install cargo-tomlfmt; \
	)

JS-BEAUTIFY-required:
	@which js-beautify > /dev/null || ( \
		echo "JS-Beautify not found. Attempting to install."; \
		yarn add js-beautify; \
	)
