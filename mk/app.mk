include $(PROJECT_DIR)/mk/common.mk

BIN_PATH ?= $(BINS_DIR)/$(APP)
LOG_FILE ?= $(ARTEFACTS)/.logs/$(APP)
PID_FILE ?= $(ARTEFACTS)/.pid/$(APP)
PKILL_PATTERN ?= $(BIN_PATH)
MODE ?= tee
SEVERITY = debug
RUST_LOG = axum=$(SEVERITY),sqlx=$(SEVERITY)

# ENVS
ENVS ?= \
    RUST_LOG='$(RUST_LOG)' \
    PG_HOST='$(PG_HOST)' \
    PG_PORT='$(PG_PORT)' \
    PG_DB='$(PG_USER_DB)' \
    PG_USER='$(PG_USER_NAME)' \
    PG_PASSWORD='$(PG_USER_PASSWORD)'

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

daemon:
	echo ENVS = $$'$(call escape,$(ENVS))' > $(LOG_FILE)
ifdef START_BIN
	$(START_BIN) >>$(LOG_FILE) 2>&1 & echo $$! > $(PID_FILE)
endif

stop:
ifdef PKILL_PATTERN
	@echo Killing $(PKILL_PATTERN) ...
	ps -A -o pid,command | grep -v grep | grep '$(PKILL_PATTERN)' | awk '{print $$1}' | xargs -I {} kill -s KILL {} || true
	@echo killed.
endif

lsof:
	lsof -nP -i4TCP@0.0.0.0:8888

run: init $(MODE)

clean:
	[ ! -f $(LOG_FILE) ] || rm -vf $(LOG_FILE)
	[ ! -f $(PID_FILE) ] || rm -vf $(PID_FILE)

distclean: clean