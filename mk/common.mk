APP ?= tetrix

# Postgresql
PG_HOST ?= localhost
PG_PORT ?= 5432
PG_USER_DB ?= $(APP)_db
PG_USER_NAME ?= $(APP)_user
PG_USER_PASSWORD ?= 12345

# sqlx
DATABASE_URL = postgres://$(PG_USER_NAME):$(PG_USER_PASSWORD)@$(PG_HOST):$(PG_PORT)/$(PG_USER_DB)

# Line Feed
define LF


endef

# SUDO
SUDO_BIN ?=
SUDO_USR ?=

# $(and ..., ..., ...) 
# - each argument is expanded, in order;
# - if an argument expands to an empty string the processing stops and the result of the expansion is the empty string;
# - if all arguments expand to a non-empty string then the result of the expansion is the expansion of the last argument;
ifneq ($(strip $(and $(SUDO_BIN),$(SUDO_USR))),)
    SUDO = $(SUDO_BIN) -u $(SUDO_USR)
else ifneq ($(strip $(SUDO_BIN)),)
    SUDO = $(SUDO_BIN)
else
    SUDO = 
endif

# Use 'abspath' instead 'realpath' because TARGET_DIR is not exists, but 'realpath' checks its existance
# $1:profile,$2:TARGET_DIR,$3:TARGET_ARCH
# EXAMPLE = $(call cargo_bins,dev,target,aarch64-apple-darwin)
define cargo_bins
$(eval 
ifeq ($1,dev)
x__PROFILE_DIR = debug
else
x__PROFILE_DIR = $1
endif)$2/$3/$(x__PROFILE_DIR)
endef

# cargo
PROFILE ?= dev
TARGET_ARCH ?= aarch64-apple-darwin
TARGET_DIR ?= $(PROJECT_DIR)/target
BINS_DIR ?= $(call cargo_bins,$(PROFILE),$(TARGET_DIR),$(TARGET_ARCH))
