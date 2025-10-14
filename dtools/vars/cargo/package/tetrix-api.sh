. ${DT_VARS}/cargo/package/defaults.sh

BINS+=(tetrix-api)

MANIFEST_DIR="${DT_PROJECT}/ttx-api"
#CLIPPY_LINTS+=("-Dwarnings")
#CLIPPY_LINTS+=("--cap-lints allow")
PACKAGE="tetrix-api"
# MANIFEST_DIR="${DT_PROJECT}"

if [ -n "${SQLX_OFFLINE+set_or_not_null}" ]; then
  add_env SQLX_OFFLINE ${SQLX_OFFLINE}
  if [ "${SQLX_OFFLINE}" = "false" ]; then
    add_env DATABASE_URL "$(. "${DT_VARS}/conns/pg_18/migrator.sh" && echo "$(pg_conn_url)")"
  fi
else
  add_env DATABASE_URL "$(. "${DT_VARS}/conns/pg_18/migrator.sh" && echo "$(pg_conn_url)")"
fi

LOCALS=${DT_LOCAL_VARS}/cargo/package/${PACKAGE}.sh
source_locals ${LOCALS}