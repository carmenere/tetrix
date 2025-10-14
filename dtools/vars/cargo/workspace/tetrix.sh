. ${DT_VARS}/cargo/workspace/defaults.sh

if [ -z "${BINS+set_or_not_null}" ]; then
    BINS+=("tetrix-api")
    BINS+=("tetrix")
else
  BINS=($(echo "${BINS[@]}"))
fi

# EXCLUDE+=()

#CLIPPY_LINTS+=("-Dwarnings")
#CLIPPY_LINTS+=("--cap-lints allow")

if [ -n "${SQLX_OFFLINE+set_or_not_null}" ]; then
  add_env SQLX_OFFLINE ${SQLX_OFFLINE}
  if [ "${SQLX_OFFLINE}" = "false" ]; then
    add_env DATABASE_URL "$(. "${DT_VARS}/conns/pg_18/migrator.sh" && echo "$(pg_conn_url)")"
  fi
else
  add_env DATABASE_URL "$(. "${DT_VARS}/conns/pg_18/migrator.sh" && echo "$(pg_conn_url)")"
fi

LOCALS=${DT_LOCAL_VARS}/cargo/workspace/tetrix.sh
source_locals ${LOCALS}