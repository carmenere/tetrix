. ${DT_VARS}/cargo/package/defaults.sh

BINS=()

MANIFEST_DIR="${DT_PROJECT}/ttx-ns"
#CLIPPY_LINTS+=("-Dwarnings")
#CLIPPY_LINTS+=("--cap-lints allow")
PACKAGE="tetrix-namespace"
# MANIFEST_DIR="${DT_PROJECT}"

LOCALS=${DT_LOCAL_VARS}/cargo/package/${PACKAGE}.sh
source_locals ${LOCALS}