include $(PROJECT_DIR)/mk/common.mk

SQLX = sqlx
BIN_PATH ?= $(SQLX) migrate run
LOG_FILE ?= $(ARTEFACTS)/.logs/$(SQLX)
PID_FILE ?= $(ARTEFACTS)/.pid/$(SQLX)
PKILL_PATTERN ?= $(BIN_PATH)
MODE ?= shell
MIGRATIONS = $(PROJECT_DIR)/migrations/schemas

# ENVS
ENVS ?= \
    DATABASE_URL='$(DATABASE_URL)'

# OPTS
OPTS ?= \
    --source $(MIGRATIONS)

define escape
$(subst ",\",$(subst ',\',$1))
endef

ifdef BIN_PATH
    START_BIN ?= $(ENVS) $(BIN_PATH) $(OPTS)
else
    START_BIN ?=
endif

.PHONY: init shell tee run clean distclean

init:
	[ -d $(dir $(LOG_FILE)) ] || mkdir -p $(dir $(LOG_FILE))
	[ -d $(dir $(PID_FILE)) ] || mkdir -p $(dir $(PID_FILE))

shell:
ifdef START_BIN
	$(START_BIN)
endif

tee:
	echo ENVS = $$'$(call escape,$(ENVS))' > $(LOG_FILE)
ifdef START_BIN
	bash -c $$'$(call escape,$(START_BIN) 2>&1 | tee -a $(LOG_FILE); exit $${PIPESTATUS[0]})'
endif

run: init $(MODE)

clean:
	[ ! -f $(LOG_FILE) ] || rm -vf $(LOG_FILE)
	[ ! -f $(PID_FILE) ] || rm -vf $(PID_FILE)

distclean: clean