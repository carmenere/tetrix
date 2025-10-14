CARGO_PACKAGES=(tetrix-api tetrix-cli tetrix-namespace)
OS_SERVICES=(pg_18)
APPS=(${CARGO_PACKAGES[@]})

autocomplete_add cmd_family_app ${APPS[@]}
autocomplete_add cmd_family_brew ${OS_SERVICES[@]}
autocomplete_add cmd_family_cargo_crates "sqlx-cli" "cargo-audit" "cargo-cyclonedx" "cargo-deny" "cargo-sonar"
autocomplete_add cmd_family_cargo_package ${CARGO_PACKAGES[@]}
autocomplete_add cmd_family_cargo_ssdlc "tetrix"
autocomplete_add cmd_family_cargo_workspace "tetrix"
autocomplete_add cmd_family_curl "version"
autocomplete_add cmd_family_docker_image ${OS_SERVICES[@]} "builder"
autocomplete_add cmd_family_docker_network "example"
autocomplete_add cmd_family_docker_service ${OS_SERVICES[@]}
autocomplete_add cmd_family_lsof ${OS_SERVICES[@]} "pg_ctl_18"
autocomplete_add cmd_family_pg_ctl "pg_ctl_18"
autocomplete_add cmd_family_pg_services "pg_18"
autocomplete_add cmd_family_psql "admin" "migrator" "app"
autocomplete_add cmd_family_psql_batch "pg_18" "pg_ctl_18"
autocomplete_add cmd_family_rustup "1.90.0"
autocomplete_add cmd_family_service ${OS_SERVICES[@]}
autocomplete_add cmd_family_sqlx "tetrix-api"
autocomplete_add cmd_family_systemctl ${OS_SERVICES[@]}
autocomplete_add cmd_family_tmux ${APPS[@]}

autocomplete_add cmd_family_m4_psql_query alter_role_password.sql drop_role_password.sql create_user.sql drop_user.sql \
  create_db.sql drop_db.sql grant_user_migrator.sql revoke_user_migrator.sql grant_user_app.sql revoke_user_app.sql \
  reinit_template1.sql

autocomplete_add cmd_family_stand "tetrix"
autocomplete_add_key "tetrix"
autocomplete_add tetrix assign_grants build clean_services clippy configure_reinit_up configure_services down fixtures \
  fmt fmt_fix init_services install_cargo_deps install_services migrations post_install_services prepare \
  prepare_reinit_up reinit_pg reinit_services reinit_up reports start_apps start_services stop_apps stop_services \
  tests up upgrades
