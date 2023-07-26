use autotools::Config;
use std::process::Command;

fn main() {
    let xen_path_str = format!("{}{}", std::env::var("OUT_DIR").unwrap(), "/xen/");
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let git_ref = std::env::var("GIT_REF").unwrap_or_else(|_| "master".to_string());

    Command::new("git")
        .args([
            "clone",
            "https://xenbits.xen.org/git-http/xen.git",
            "--single-branch",
            "--branch",
            &git_ref,
            "--depth",
            "1",
            "--shallow-submodules",
            &xen_path_str,
        ])
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    Command::new("git")
        .args(["-C", &xen_path_str, "pull"])
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    // ./configure --build=x86_64-unknown-linux-gnu --host=aarch64-linux-gnu \
    // --disable-docs --disable-golang --disable-ocamltools \
    // --with-system-qemu=/usr/bin/qemu-system-i386

    std::env::set_current_dir(&xen_path_str).unwrap();

    let _build_result = Config::new(&xen_path_str)
        .out_dir(&xen_path_str)
        .forbid("--prefix")
        .forbid("--host_alias")
        .forbid("--cache-file")
        .forbid("--srcdir")
        .forbid("--disable-shared")
        .forbid("--enable-static")
        .insource(true)
        .disable("docs", None)
        .disable("golang", None)
        .disable("ocamltools", None)
        .config_option("with-system-qemu", Some("/usr/bin/qemu-system-i386"))
        .config_option("build", Some("x86_64-unknown-linux-gnu"))
        .config_option("host", Some("aarch64-linux-gnu"))
        .make_target("dist")
        .build();
    Command::new(format!("{crate_dir}/split.sh"))
        .env_clear()
        .env("XEN_DIR", &xen_path_str)
        .current_dir(&crate_dir)
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
}
