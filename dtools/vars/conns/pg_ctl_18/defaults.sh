. <(set -eu; . ${DT_VARS}/services/pg_ctl_18.sh
  echo "MODE=${MODE}"
  echo "SERVICE=${SERVICE}"
  echo "port_app=${PORT_CONN}"
  echo "port_client=${PORT_BIND}"
  echo "host=${HOST_CONN}"
  echo "EXEC=${EXEC}"
  echo "TERMINAL=${TERMINAL}"
  echo "CLIENT=${CLIENT}"
)

if [ "${MODE}" = "docker" ]; then
  dt_error 'defaults' "The mode ${BOLD}docker${RESET} is incompatible with ${BOLD}pg_ctl${RESET}."
  return 99
fi
