include $(PROJECT_DIR)/mk/common.mk

ADMIN ?= an.romanov
ADMIN_DB ?= postgres
ADMIN_PASSWORD ?= postgres
AUTH_METHOD ?= remote
# Docker container name. So, you can run postgres inside container.
CONTAINER = 
EXIT_IF_DB_EXISTS = no
EXIT_IF_USER_EXISTS = no
USER_ATTRIBUTES ?= SUPERUSER CREATEDB
REVOKE_ATTRS ?= 
PG_DUMP ?= /tmp/.dumps/$(PG_USER_DB).sql

CONN_URL ?= postgres://$(ADMIN):$(ADMIN_PASSWORD)@$(PG_HOST):$(PG_PORT)/$(ADMIN_DB)
USER_CONN_URL ?= $(DATABASE_URL)

define select_user
SELECT '$1' FROM pg_roles WHERE rolname = '$1'
endef

define select_db
SELECT '$1' FROM pg_database WHERE datname = '$1'
endef

define check
$$($(PSQL) -tXAc $$'$(subst ',\',$(call select_$1,$2))')
endef

#
ifdef CONTAINER
    PSQL ?= docker exec $(TI) $(CONTAINER) psql -U $(ADMIN) -d $(ADMIN_DB)
    PSQL_USER ?= docker exec $(TI) $(CONTAINER) psql -U $(PG_USER_NAME) -d $(PG_USER_DB)
else ifeq ($(AUTH_METHOD),remote)
    PSQL = psql $(CONN_URL)
    PSQL_USER ?= psql $(USER_CONN_URL)
else ifeq ($(AUTH_METHOD),peer)
    PSQL ?= $(SUDO) -iu $(ADMIN) PGDATABASE=$(ADMIN_DB) psql
    PSQL_USER ?= $(SUDO) -iu $(PG_USER_NAME) PGDATABASE=$(PG_USER_DB) psql
else
    $(error Unsupported value '$(AUTH_METHOD)' for 'AUTH_METHOD' variable. SECTION=$(SECTION))
endif

# Targets

.PHONY: init create-user create-db grant revoke connect connect-admin clear clean dump distclean import lsof

create-user:
ifeq ($(EXIT_IF_USER_EXISTS),yes)
	[ -z "$(call check,user,$(PG_USER_NAME))" ] || false
endif
	[ -n "$(call check,user,$(PG_USER_NAME))" ] || $(PSQL) -c "CREATE USER $(PG_USER_NAME) WITH ENCRYPTED PASSWORD '$(PG_USER_PASSWORD)' $(USER_ATTRIBUTES);"

create-db: create-user
ifeq ($(EXIT_IF_DB_EXISTS),yes)
	[ -z "$(call check,db,$(PG_USER_DB))" ] || false
endif
	[ -n "$(call check,db,$(PG_USER_DB))" ] || $(PSQL) -c "CREATE DATABASE $(PG_USER_DB) WITH OWNER=$(PG_USER_NAME);"

grant: create-db
	# Assign priviliges to user '$(PG_USER_NAME)'
	$(PSQL) -c "GRANT ALL PRIVILEGES ON DATABASE $(PG_USER_DB) TO $(PG_USER_NAME);"
	$(PSQL) -c "GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO $(PG_USER_NAME);"

revoke:
	$(foreach A,$(REVOKE_ATTRS),$(PSQL) -c "ALTER USER $(PG_USER_NAME) WITH NO$(ATTRIBUTE);" $(LF))

init: create-user create-db grant

connect: override TI = -ti
connect:
	$(PSQL_USER)

connect-admin: override TI = -ti
connect-admin:
	$(PSQL)

dump:
	PGPASSWORD=$(PG_USER_PASSWORD) pg_dump -h $(PG_HOST) -p $(PG_PORT) -U $(PG_USER_NAME) -d $(PG_USER_DB) --file=$(PG_DUMP)

import: clean init
	$(USER_URL) --set ON_ERROR_STOP=on -f "$(PG_DUMP)"

clear:
	$(PSQL_USER) -c "DROP SCHEMA IF EXISTS public CASCADE;"
	$(PSQL_USER) -c "CREATE schema public;"

clean:
	$(PSQL) -c "DROP DATABASE IF EXISTS $(PG_USER_DB);"
	$(PSQL) -c "DROP USER IF EXISTS $(PG_USER_NAME);"

distclean: clean

lsof:
ifneq ($(PG_HOST),0.0.0.0)
	sudo lsof -nP -i4TCP@0.0.0.0:$(PG_PORT) || true
endif
ifneq ($(PG_HOST),localhost)
	sudo lsof -nP -i4TCP@localhost:$(PG_PORT) || true
endif
	sudo lsof -nP -i4TCP@$(PG_HOST):$(PG_PORT) || true