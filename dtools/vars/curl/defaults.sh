. <(. "${DT_VARS}/apps/tetrix.sh"
  echo "API_HOST=${envs["TTX_BIND_HOST"]}"
  echo "API_PORT=${envs["TTX_BIND_PORT"]}"
)

HEADERS=()
HEADERS+=("Content-Type: application/json")
