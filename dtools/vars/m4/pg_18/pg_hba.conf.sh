. <(
  . ${DT_VARS}/services/pg_18.sh
  echo "M4_OUT=\"$(pg_pg_hba.conf)\""
  echo "M4_HBA_POLICY=\"${PG_HBA_POLICY}\""
)

declare -A envs=()
declare -a ENVS=()

add_env M4_HBA_POLICY "${M4_HBA_POLICY}"

M4_IN="${DT_M4}/pg/pg_hba.conf"
M4_TVARS="${M4_IN}.m4"
