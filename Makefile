
# Keep marker directories.
.SECONDARY:

# Find all examples, e.g. "hello-name"
EXAMPLES := $(shell find . -type f -name Cargo.toml | awk -F / '{print $$2}' | uniq)
# Find all contracts, e.g. "hello-name/call" and "hello-name/define"
CONTRACTS := $(shell find . -type f -name Cargo.toml | sed 's/\/Cargo.toml//' | sed 's/.\///')

RUST_TOOLCHAIN := $(shell cat rust-toolchain)

all: $(EXAMPLES)

clean: down $(shell find . -type f -name "Cargo.toml" | awk '{print $$1"/clean"}')
	rm -rf .make

# Defining rules so each target building a contract only depends on its own source. Needs GNU Make.
# Building either a call or a define, leaving the WASM files in <contract>/target/wasm32-unknown-unknown/release/
define CONTRACT_rule
.make/contracts/$(1): $$(shell find $(1) -type f \( -name "Cargo.toml" -o -wholename "*/src/*.rs" \)) .make/rustup-update
	cd $(1) && cargo +$(RUST_TOOLCHAIN) build --release --target wasm32-unknown-unknown
	mkdir -p $$(dir $$@) && touch $$@
endef

# Building both call and define in an example.
define EXAMPLE_rule
.PHONY: $(1)
$(1): .make/contracts/$(1)/call .make/contracts/$(1)/define
endef

$(foreach d,$(EXAMPLES),$(eval $(call EXAMPLE_rule,$(d))))
$(foreach d,$(CONTRACTS),$(eval $(call CONTRACT_rule,$(d))))


%/Cargo.toml/clean:
	cd $* && cargo clean


.make/rustup-update: rust-toolchain
	rustup update $(RUST_TOOLCHAIN)
	rustup toolchain install $(RUST_TOOLCHAIN)
	rustup target add --toolchain $(RUST_TOOLCHAIN) wasm32-unknown-unknown
	mkdir -p $(dir $@) && touch $@


up:
	docker-compose up -d

down:
	docker-compose down
