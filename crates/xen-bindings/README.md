# xen-bindings

Rust FFI bindings for the Xen hypervisor and xentools generated using [bindgen](https://crates.io/crates/bindgen).

## Usage

Add this to your `Cargo.toml`:

```toml
xen-bindings = { git = "https://github.com/rust-vmm/xen-sys/", version = "0.1.0" }
```

You can then import the bindings where you need them:

```rust
use xen_bindings::bindings::
```

or

```rust
use xen_bindings::bindings::{xs_watch_type, xs_watch_type_XS_WATCH_PATH};
```

## Configuration

The default behavior of this crate is to fetch `xen` sources and build bindings
from `xen`'s generated headers. This requires extra system dependencies.
`FIXME: which ones?`

If you prefer to use the pre-built bindings, activate the Cargo feature `bundled`:

```toml
xen-bindings = { git = "https://github.com/rust-vmm/xen-sys/", version = "0.1.0", features = ["bundled"] }
```

The building behavior can be configured with the following environment flags:

- `GIT_REF`: the `git` commit, branch or tag to fetch. Default is `master`.
- `GIT_URL`: the `git` repository URL to fetch from. Default is
  `https://xenbits.xen.org/git-http/xen.git`.
- `XEN_SRC_PATH`: optional path to a local copy of `xen` sources. If set, the
  build script will use it instead of `git-clone`ing from the network.
