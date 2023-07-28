#!/bin/bash -ex

# x86_64 or aarch64
if [[ -z "${ARCH}" ]]; then
  ARCH="x86_64"
fi

# Path to bindgen binary
if [[ -z "${BINDGEN}" ]]; then
  BINDGEN="bindgen"
fi

# for debug logs
pwd

printf "ARCH    = '%s'\n" "${ARCH}"
printf "BINDGEN = '%s'\n" "${BINDGEN}"
printf "XEN_DIR = '%s'\n" "${XEN_DIR}"

# Path to Xen project source code
if [[ -z "${XEN_DIR}" ]]; then
  echo "Environment variable XEN_DIR must be set to an existing xen repository"
  exit 1
fi

if [[ "${ARCH}" = "x86_64" ]]; then
  if [[ -z "${XEN_DIR_X86}" ]]; then
    printf "Environment variable XEN_DIR_X86 not set, using fallback value XEN_DIR=%s\n" "${XEN_DIR}"
    XEN_DIR_X86="${XEN_DIR}"
  fi

  for d in "/xen/include/" "/xen/include/public/" "/dist/install/usr/local/include/" "/dist/install/usr/local/include/xen/sys/"; do
    echo  "${XEN_DIR_X86}""${d}"
    if [[ ! -d "${XEN_DIR_X86}${d}" ]]; then
      echo "${XEN_DIR_X86}${d}" does not exist. Verify that "${XEN_DIR_X86}" is correctly set.
      exit 1
    fi
  done

	"${BINDGEN}" wrapper_xen_x86_64.h -o src/xen_bindings_xen_x86_64.rs \
	--ignore-functions \
	--ignore-methods \
	--no-layout-tests \
	--use-core \
	--ctypes-prefix=xen_bindings_x86_64_types \
	-- \
	-D__XEN_TOOLS__ \
	-D__x86_64__ \
	-I"${XEN_DIR_X86}"/xen/include/ \
	-I"${XEN_DIR_X86}"/xen/include/public/

  echo "Wrote x86_64 bindings to src/xen_bindings_xen_x86_64.rs"

	"${BINDGEN}" wrapper_tools_x86_64.h -o src/xen_bindings_tools_x86_64.rs \
	--ignore-functions \
	--ignore-methods \
	--no-layout-tests \
	--use-core \
	--ctypes-prefix=xen_bindings_x86_64_types \
	--allowlist-file=.*xenctrl.h \
	--allowlist-file=.*xendevicemodel.h \
	--allowlist-file=.*xengnttab.h \
	--allowlist-file=.*xenstore.h \
	-- \
	-D__XEN_TOOLS__ \
	-I"${XEN_DIR_X86}"/dist/install/usr/local/include/ \
	-I"${XEN_DIR_X86}"/dist/install/usr/local/include/xen/sys/

  echo "Wrote x86_64 tool bindings to src/xen_bindings_tools_x86_64.rs"

elif [[ "${ARCH}" = "aarch64" ]]; then
  if [[ -z "${XEN_DIR_AARCH64}" ]]; then
    printf "Environment variable XEN_DIR_AARCH64 not set, using fallback value XEN_DIR=%s\n" "${XEN_DIR}"
    XEN_DIR_AARCH64="${XEN_DIR}"
  fi

  for d in "/xen/include/" "/xen/include/public/" "/dist/install/usr/local/include/" "/dist/install/usr/local/include/xen/sys/"; do
    echo  "${XEN_DIR_AARCH64}""${d}"
    if [[ ! -d "${XEN_DIR_AARCH64}${d}" ]]; then
      echo "${XEN_DIR_AARCH64}${d}" does not exist. Verify that "${XEN_DIR_AARCH64}" is correctly set.
      exit 1
    fi
  done

	"${BINDGEN}" wrapper_xen_aarch64.h -o src/xen_bindings_xen_aarch64.rs \
	--ignore-functions \
	--ignore-methods \
	--no-layout-tests \
	--use-core \
	--ctypes-prefix=xen_bindings_aarch64_types \
	-- \
	-U__i386__ \
	-U__x86_64__ \
	-D__aarch64__ \
	-DCONFIG_ARM_64 \
	-D__XEN_TOOLS__ \
	-I"${XEN_DIR_AARCH64}"/xen/include/ \
	-I"${XEN_DIR_AARCH64}"/xen/include/public/

  echo "Wrote aarch64 bindings to src/xen_bindings_xen_aarch64.rs"

	"${BINDGEN}" wrapper_tools_aarch64.h -o src/xen_bindings_tools_aarch64.rs \
	--ignore-functions \
	--ignore-methods \
	--no-layout-tests \
	--use-core \
	--ctypes-prefix=xen_bindings_aarch64_types \
	--allowlist-file=.*xenctrl.h \
	--allowlist-file=.*xendevicemodel.h \
	--allowlist-file=.*xengnttab.h \
	--allowlist-file=.*xenstore.h \
	-- \
	-D__aarch64__ \
	-DCONFIG_ARM_64 \
	-D__XEN_TOOLS__ \
	-I"${XEN_DIR_AARCH64}"/dist/install/usr/local/include/ \
	-I"${XEN_DIR_AARCH64}"/dist/install/usr/local/include/xen/sys/

  echo "Wrote aarch64 tool bindings to src/xen_bindings_tools_aarch64.rs"
else
  printf "Generated no bindings. Given ARCH value is '%s'\n" "${ARCH}"
  exit 1
fi
