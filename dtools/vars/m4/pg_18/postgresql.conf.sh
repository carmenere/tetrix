. <(
  . ${DT_VARS}/services/pg_18.sh
  echo "M4_PORT=\"${PORT_BIND}\""
  echo "M4_PG_DATADIR=\"$(pg_data_directory)\""
  echo "M4_HBA_CONF=\"$(pg_pg_hba.conf)\""
  echo "M4_OUT=$(pg_postgresql.conf)"
)

declare -A envs=()
declare -a ENVS=()

add_env M4_PORT "${M4_PORT}"
add_env M4_PG_DATADIR "${M4_PG_DATADIR}"
add_env M4_HBA_CONF "${M4_HBA_CONF}"

M4_IN="${DT_M4}/pg/postgresql.conf"
M4_TVARS="${M4_IN}.m4"
