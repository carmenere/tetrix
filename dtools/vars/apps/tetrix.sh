declare -A envs
ENVS=()

. <(. "${DT_VARS}/cargo/package/tetrix-cli.sh"
  echo "BINS_DIR=\"${BINS_DIR}\""
)

APP="tetrix"
OPTS=
BINARY="${BINS_DIR}/${APP}"

LOCALS=${DT_LOCAL_VARS}/apps/${APP}.sh
source_locals ${LOCALS}
