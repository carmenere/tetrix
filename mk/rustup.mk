include $(PROJECT_DIR)/mk/common.mk

CRATES ?= cargo-cache__0.8.3 sqlx-cli__0.7.4 cargo-pgrx__0.11.4 cargo-get__1.1.1
RUST_VERSION ?= 1.80.0
RUSTFLAGS ?= -C target-feature=-crt-static
SOURCE_ENV ?= source "${HOME}/.cargo/env"
FORCE ?= no
DEFAULT_TOOLCHAIN ?= no
COMPONENTS += clippy
COMPONENTS += rustfmt

# OPT_FORCE
ifeq ($(FORCE),yes)
    OPT_FORCE = --force
else
    OPT_FORCE = 
endif

define LF


endef

.PHONY: all init nightly toolchain components crates clean distclean

all: nightly init toolchain components crates

init:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $(RUST_VERSION)-$(TARGET_ARCH)

nightly:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-$(TARGET_ARCH)

toolchain:
	$(SOURCE_ENV) && rustup toolchain install $(RUST_VERSION)-$(TARGET_ARCH)
ifeq ($(DEFAULT_TOOLCHAIN),yes)
	$(SOURCE_ENV) && rustup default $(RUST_VERSION)-$(TARGET_ARCH)
endif

components:
	$(SOURCE_ENV) && RUSTFLAGS='$(RUSTFLAGS)' rustup +$(RUST_VERSION) component add $(COMPONENTS)

crates:
	$(foreach CRATE,$(CRATES),$(SOURCE_ENV) && \
		RUSTFLAGS='$(RUSTFLAGS)' cargo +$(RUST_VERSION) install --target=$(TARGET_ARCH) --version=$(lastword $(subst __, ,$(CRATE))) $(OPT_FORCE) $(firstword $(subst __, ,$(CRATE))) \
	$(LF))

clean:
	cargo +$(RUST_VERSION) clean
	cargo +$(RUST_VERSION) cache -r all

distclean: clean
