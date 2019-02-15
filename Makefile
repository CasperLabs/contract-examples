
# Keep marker directories.
.SECONDARY:

all: counter hello-name mailing-list


clean: $(shell find . -type f -name "Cargo.toml" | awk '{print $$1"/clean"}')
	rm -rf .make


%/Cargo.toml/clean:
	cd $* && cargo clean


.PHONY: hello-name
hello-name: .make/contracts/hello-name/call .make/contracts/hello-name/define

.PHONY: counter
counter: .make/contracts/counter/call .make/contracts/counter/define

.PHONY: mailing-list
mailing-list: .make/contracts/mailing-list/call .make/contracts/mailing-list/define


# Build a `call` or `define` contract, depends on any source file under it.
.make/contracts/%: $(shell find $* -type f -iregex ".*/Cargo\.toml\|.*\.rs") .make/rustup-update
	cd $* && cargo build --release --target wasm32-unknown-unknown
	@# Built the WASM files in <contract>/target/wasm32-unknown-unknown/release/
	mkdir -p $(dir $@) && touch $@


.make/rustup-update:
	rustup update
	rustup target add wasm32-unknown-unknown
	mkdir -p $(dir $@) && touch $@