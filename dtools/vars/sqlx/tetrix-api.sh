. ${DT_VARS}/sqlx/defaults.sh

APP_ID="tetrix-api"
SCHEMAS="${DT_PROJECT}/ttx-api/migrations/schemas"
TMP_SCHEMAS="${DT_ARTEFACTS}/schemas"
SQLX_RUN_HOOK=

add_env DATABASE_URL "$(. "${DT_VARS}/conns/pg_18/migrator.sh" && echo "$(pg_conn_url)")"

dt_debug "sdfdsf" "dsfdsf"