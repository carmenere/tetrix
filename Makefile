SHELL := bash
SELF := $(realpath $(lastword $(MAKEFILE_LIST)))

export PROJECT_DIR := $(realpath $(dir $(SELF)))
export ARTEFACTS := $(PROJECT_DIR)/.artefacts

MK := $(PROJECT_DIR)/mk

.PHONY: init-db clean-db reinit-db rustup build checks start stop tests

init-db:
	make -f $(MK)/psql.mk init

clean-db:
	make -f $(MK)/psql.mk clean

reinit-db: clean-db init-db

connect-db:
	make -f $(MK)/psql.mk connect

rustup:
	make -f $(MK)/rustup.mk all

init: init-db rustup

checks:
	make -f $(MK)/cargo.mk fmt-check lint

fmt-check:
	make -f $(MK)/cargo.mk fmt-check

fmt:
	make -f $(MK)/cargo.mk fmt

schemas: init-db
	make -f $(MK)/sqlx.mk run

build: schemas
	make -f $(MK)/cargo.mk build

restart: stop build
	make -f $(MK)/app.mk run

start: build
	make -f $(MK)/app.mk run

stop:
	make -f $(MK)/app.mk stop

tests: stop build
	make -f $(MK)/app.mk daemon
	sleep 5
	make -f $(MK)/cargo.mk test

docs:
	make -f $(MK)/cargo.mk doc

docs-open:
	make -f $(MK)/cargo.mk doc-open