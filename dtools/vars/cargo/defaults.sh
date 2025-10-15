# PROFILE = { release | dev }, by default "dev"
# BUILD_AS = { package | workspace | }, by default "package", if empty, then just "--bin xxx" without --package or --workspace

declare -A envs=()
declare -a ENVS=()

. <(. ${DT_VARS}/rustup/1.90.0.sh
  echo "RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN}"
  echo "NIGHTLY_VERSION=${NIGHTLY_VERSION}"
)

#BINS=()
BUILD_AS="package"
CARGO_BUILD_TARGET=
CARGO_TARGET_DIR="$(realpath "$(dt_mkdir ${DT_ARTEFACTS}/target)")"

CLIPPY_LINTS=()
PROFILE="dev"
RUSTFLAGS=
FEATURES=()

CLIPPY_REPORT=
EXCLUDE=()
MANIFEST="Cargo.toml"
MANIFEST_DIR="${DT_PROJECT}"
MESSAGE_FORMAT=
PACKAGE=

# Depends on PROFILE
BUILD_MODE=$(cg_build_mode)
# Depends on CARGO_TARGET_DIR, BUILD_MODE
BINS_DIR=$(cg_bin_dir)

# cargo envs
add_env CARGO_TARGET_DIR "${CARGO_TARGET_DIR}"
add_env RUSTUP_TOOLCHAIN "${RUSTUP_TOOLCHAIN}"
