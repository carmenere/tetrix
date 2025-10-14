. ${DT_VARS}/docker_images/defaults.sh

SUBNET="192.168.99.0/24"
BRIDGE="example"
DRIVER="bridge"

LOCALS=${DT_LOCAL_VARS}/docker_bridges/example.sh
source_locals ${LOCALS}