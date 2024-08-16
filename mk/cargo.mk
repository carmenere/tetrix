include $(PROJECT_DIR)/mk/common.mk

PACKAGE ?= $(APP)
BINS ?= $(APP)
CLIPPY_FORMAT ?= json
CLIPPY_REPORT ?= clippy-report.json
FEATURES ?= 
INSTALL_DIR ?= /usr/local/bin
LINTS ?= 
MANIFEST ?= Cargo.toml
RUST_VERSION ?= 1.80.0

# BUILD ENVS
CARGO_TOML ?= $(PROJECT_DIR)/$(MANIFEST)
BUILD_VERSION = $(shell git log -1 --pretty=format:"%h")
# RUSTFLAGS = -C target-feature=-crt-static

BUILD_ENVS ?= \
    RUSTFLAGS='$(RUSTFLAGS)' \
    BUILD_VERSION='$(BUILD_VERSION)' \
    DATABASE_URL='$(DATABASE_URL)'

# OPT_BINS
ifdef BINS
    OPT_BINS = $(foreach BIN,$(BINS), --bin $(BIN))
else
    OPT_BINS =
endif

# OPT_PROFILE
ifeq ($(PROFILE),release)
    OPT_PROFILE = --profile release
else
    OPT_PROFILE = --profile dev
endif

# OPT_FEATURES
ifdef FEATURES
    OPT_FEATURES = --features $(FEATURES)
else
    OPT_FEATURES =
endif

# CARGO_OPTS
CARGO_OPTS ?= $(OPT_PROFILE) $(OPT_BINS) $(OPT_FEATURES) \
    --manifest-path $(CARGO_TOML) \
    --target-dir $(TARGET_DIR) \
    --target $(TARGET_ARCH)

CARGO_TEST_OPTS ?= $(OPT_PROFILE) $(OPT_FEATURES) \
    --manifest-path $(CARGO_TOML) \
    --target-dir $(TARGET_DIR) \
    --target $(TARGET_ARCH)

CARGO ?= cargo +$(RUST_VERSION)
CMD_BUILD ?= $(BUILD_ENVS) $(CARGO) build $(CARGO_OPTS)
CMD_CLIPPY ?= $(BUILD_ENVS) $(CARGO) clippy $(CARGO_OPTS) --message-format $(CLIPPY_FORMAT) -- $(LINTS) 1>$(CLIPPY_REPORT)
CMD_CLIPPY_FIX ?= $(BUILD_ENVS) $(CARGO) clippy --fix --allow-staged $(CARGO_OPTS)
CMD_TEST ?= $(BUILD_ENVS) $(CARGO) test $(CARGO_TEST_OPTS)
CMD_CLEAN ?= $(BUILD_ENVS) $(CARGO) clean --manifest-path $(CARGO_TOML) --target-dir $(TARGET_DIR)
CMD_FMT ?= $(BUILD_ENVS) cargo +nightly fmt
CMD_FMT_CHECK ?= $(CMD_FMT) -- --check && echo -e "    \033[1;32mFinished\033[0m fmt check."
CMD_DOC ?= $(BUILD_ENVS) $(CARGO) doc --no-deps --document-private-items

.PHONY: all build clippy clippy-fix lint test fmt fmt-check doc install uninstall clean distclean

all: fmt clippy test build

build:
	cd $(PROJECT_DIR) && $(CMD_BUILD)

clippy:
	cd $(PROJECT_DIR) && $(CMD_CLIPPY)

clippy-fix:
	cd $(PROJECT_DIR) && $(CMD_CLIPPY_FIX)

lint: clippy

test:
	cd $(PROJECT_DIR) && $(CMD_TEST)

fmt:
	cd $(PROJECT_DIR) && $(CMD_FMT)

fmt-check:
	cd $(PROJECT_DIR) && $(CMD_FMT_CHECK)

doc:
	cd $(PROJECT_DIR) && $(CMD_DOC)

clean:
	cd $(PROJECT_DIR) && $(CMD_CLEAN)

distclean: clean

install:
	$(SUDO) install -d $(INSTALL_DIR)
	$(foreach BIN,$(BINS),$(SUDO) install -m 755 -t $(INSTALL_DIR) $(BINS_DIR)/$(BIN) $(LF))

uninstall:
	$(foreach BIN,$(BINS),$(SUDO) rm $(INSTALL_DIR)/$(BIN) $(LF))