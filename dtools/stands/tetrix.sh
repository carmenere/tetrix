CARGO_CRATES=(cargo-audit cargo-cyclonedx cargo-deny cargo-sonar sqlx-cli)
CARGO_PACKAGES=(tetrix-api)
SERVICES=(pg_18)
TMUX_APPS=(tetrix-api)

install_services() {
  install_postgres pg_18
  post_install_services
}

post_install_services() {
  post_install_postgres pg_18
  # install_postgis pg_18
}

install_cargo_deps() {
  local crate;
  rustup_init 1.90.0
  . "${HOME}/.cargo/env" || return $?
  for crate in "${CARGO_CRATES[@]}"; do cargo_install ${crate}; done;
}

start_services() { local srv; for srv in "${SERVICES[@]}"; do service_start ${srv}; done; }
stop_services() { local srv; for srv in "${SERVICES[@]}"; do service_stop ${srv}; done; }

configure_services() {
  pg_prepare pg_18
  start_services
  service_check psql_check pg_18 admin
  psql_alter_role_password pg_18 admin
}

init_services() {
  psql_init pg_18
}

clean_services() {
  psql_clean pg_18
}

migrations() { sqlx_run tetrix-api; }
fixtures() { true; }
upgrades() { true; }

assign_grants() { psql_grant_user pg_18 app; }
start_apps() { local app; reinit_logs && for app in "${TMUX_APPS[@]}"; do tmux_start ${app}; done; }
stop_apps() { local app; for app in "${TMUX_APPS[@]}"; do tmux_stop ${app}; done; }

clippy() { local app; for app in "${CARGO_PACKAGES[@]}"; do cargo_clippy package ${app}; done; }
build() { local app; for app in "${CARGO_PACKAGES[@]}"; do cargo_build package ${app}; done; }
fmt() { local app; for app in "${CARGO_PACKAGES[@]}"; do cargo_fmt package ${app}; done; }
fmt_fix() { local app; for app in "${CARGO_PACKAGES[@]}"; do cargo_fmt_fix package ${app}; done; }

prepare() {(
  set -eu
  install_cargo_deps
  configure_services
)}

reinit_services() {(
  set -eu
  clean_services
  init_services
  migrations
  assign_grants
)}

up() {(
  set -eu
  stop_apps
  tmux_kill
  cargo_all workspace tetrix
  fixtures
  start_apps
)}

reinit_pg() {(
  set -eu
  stop_apps
  tmux_kill
  psql_reinit pg_18
  migrations
  assign_grants
  cargo_load_env
  fixtures
)}

reinit_up() {(
  set -eu
  stop_apps
  tmux_kill
  reinit_services
  cargo_all workspace tetrix
  fixtures
  start_apps
)}

configure_reinit_up() {(
  set -eu
  stop_apps
  tmux_kill
  configure_services
  reinit_services
  cargo_all workspace tetrix
  fixtures
  start_apps
)}

prepare_reinit_up() {(
  set -eu
  stop_apps
  tmux_kill
  prepare
  reinit_services
  cargo_all workspace tetrix
  fixtures
  start_apps
)}

down() {(
  set -eu
  stop_apps
  tmux_kill
)}

tests() {
  stand tetrix prepare_reinit_up
  reinit_reports
  # command to run integration tests
}

reports() {
  # command to gather and send integration tests reports
}

LOCALS=${DT_LOCAL_VARS}/stands/tetrix.sh
source_locals ${LOCALS}