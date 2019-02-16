
# Keep marker directories.
.SECONDARY:


# Defining rules so each target building a contract only depends on its own source. Needs GNU Make.
# Building either a call or a define, leaving the WASM files in <contract>/target/wasm32-unknown-unknown/release/
# e.g. hello-name/call or hello-name/define
define CONTRACT_rule
.make/contracts/$(1): $$(shell find $(1) -type f -iregex ".*/Cargo\.toml\|.*/src/.*\.rs") .make/rustup-update
	cd $(1) && cargo build --release --target wasm32-unknown-unknown
	mkdir -p $$(dir $$@) && touch $$@
endef

# Building one of the examples, e.g. hello-name
define EXAMPLE_rule
.PHONY: $(1)
$(1): .make/contracts/$(1)/call .make/contracts/$(1)/define
endef

EXAMPLES := $(shell find . -type f -name Cargo.toml | awk -F / '{print $$2}' | uniq)
CONTRACTS := $(patsubst src/%,%,$(shell find . -type f -name Cargo.toml | sed 's/\/Cargo.toml//' | sed 's/.\///'))

all: $(EXAMPLES)

clean: $(shell find . -type f -name "Cargo.toml" | awk '{print $$1"/clean"}')
	rm -rf .make

$(foreach d,$(EXAMPLES),$(eval $(call EXAMPLE_rule,$(d))))
$(foreach d,$(CONTRACTS),$(eval $(call CONTRACT_rule,$(d))))


%/Cargo.toml/clean:
	cd $* && cargo clean


.make/rustup-update:
	rustup update
	rustup target add wasm32-unknown-unknown
	mkdir -p $(dir $@) && touch $@