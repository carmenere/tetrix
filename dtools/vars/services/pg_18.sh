MODE=host

MAJOR=18
MINOR=5
POSTGIS_VERSION="3.6.0"
LIBINTL_VERSION="0.26"
SERVICE="pg_18"

OS_SERVICE=$(pg_service)
BIN_DIR=$(pg_bin_dir)

PG_HBA_POLICY='host all all 0.0.0.0/0 md5'

HOST_BIND="localhost"
PORT_BIND="5432"
HOST_CONN="${HOST_BIND}"
PORT_CONN="${PORT_BIND}"

CLIENT="${BIN_DIR}/psql"

EXEC="host_exec"
TERMINAL="host_exec"

LOCALS=${DT_LOCAL_VARS}/services/${SERVICE}.sh
source_locals ${LOCALS}