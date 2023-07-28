#[cfg(not(feature = "bundled"))]
mod build {
    use std::env;
    use std::path::Path;
    use std::process::{Command, Stdio};

    use autotools::Config;

    fn env_inner(name: &str) -> Option<String> {
        let var = env::var(name).ok();
        println!("cargo:rerun-if-env-changed={}", name);

        match var {
            Some(ref v) => println!("{name} = {v}"),
            None => println!("{name} unset"),
        }

        var
    }

    fn env(name: &str) -> Option<String> {
        let prefix = env::var("TARGET").unwrap().to_uppercase().replace('-', "_");
        let prefixed = format!("{}_{}", prefix, name);
        env_inner(&prefixed).or_else(|| env_inner(name))
    }

    pub(super) fn build() {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let git_ref = env("GIT_REF").unwrap_or_else(|| "master".to_string());
        let git_url = env("GIT_URL")
            .unwrap_or_else(|| "https://xenbits.xen.org/git-http/xen.git".to_string());
        let local_xen_path_str = env("XEN_SRC_PATH");

        let xen_path_str = if let Some(xen_path_str) = local_xen_path_str {
            xen_path_str
        } else {
            let xen_path_str = format!("{}{}", &out_dir, "/xen/");
            if matches!(Path::try_exists(&Path::new(&xen_path_str)), Ok(true)) {
                std::fs::remove_dir_all(&xen_path_str)
                    .expect("xen repository output path already exists and could not be removed.");
            }

            let git_args = &[
                "clone",
                &git_url,
                "--single-branch",
                "--branch",
                &git_ref,
                "--depth",
                "1",
                "--shallow-submodules",
                &xen_path_str,
            ];
            let output = Command::new("git")
                .args(git_args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap();
            if !output.status.success() {
                panic!("Could not perform git {:?}: process {exit_code}.\n\nstdout: {stdout}\n\nstderr: {stderr}\n",
                    git_args,
                    exit_code = match output.status.code() {
                        Some(code) => format!("exited with status code: {code}"),
                        None       => format!("terminated by signal")
                    },
                    stdout = String::from_utf8_lossy(&output.stdout),
                    stderr = String::from_utf8_lossy(&output.stderr),
                );
            }

            let git_args = &["-C", &xen_path_str, "pull"];
            let output = Command::new("git")
                .args(git_args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap()
                .wait_with_output()
                .unwrap();
            if !output.status.success() {
                panic!("Could not perform git {:?}: process {exit_code}.\n\nstdout: {stdout}\n\nstderr: {stderr}\n",
                    git_args,
                    exit_code = match output.status.code() {
                        Some(code) => format!("exited with status code: {code}"),
                        None       => format!("terminated by signal")
                    },
                    stdout = String::from_utf8_lossy(&output.stdout),
                    stderr = String::from_utf8_lossy(&output.stderr),
                );
            }
            xen_path_str
        };

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

        let output = Command::new(format!("{crate_dir}/split.sh"))
            .env("XEN_DIR", &xen_path_str)
            .current_dir(&crate_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();
        if !output.status.success() {
            panic!("Could not build bindings: bindgen script {exit_code}.\n\nstdout: {stdout}\n\nstderr: {stderr}\n",
                    exit_code = match output.status.code() {
                        Some(code) => format!("exited with status code: {code}"),
                        None       => format!("terminated by signal")
                    },
                    stdout = String::from_utf8_lossy(&output.stdout),
                    stderr = String::from_utf8_lossy(&output.stderr),
                );
        }
    }
}

fn main() {
    #[cfg(not(feature = "bundled"))]
    build::build();
}
