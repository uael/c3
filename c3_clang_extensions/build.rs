use std::process;
use std::env;
use std::path::{PathBuf, Path};
use std::fs;

extern crate gcc;

fn main() {
    let llvm_config_cmd = env::var("LLVM_CONFIG_PATH").unwrap_or("llvm-config".to_string());
    let cmd_out = process::Command::new(llvm_config_cmd).arg("--includedir")
        .output()
        .expect("llvm-config must be in PATH or set LLVM_CONFIG_PATH to the llvm-config binary");
    let path = String::from_utf8(cmd_out.stdout).expect("utf8");
    let path = path.trim_right();

    gcc::Config::new()
                .cpp(true)
                .flag("-std=c++11")
                .flag("-Wno-comment")
                .file("src/extensions.cpp")
                .include(path)
                .include(find_clang_include())
                .include(fs::canonicalize("vendor").unwrap())
                .compile("libc3_clang_extensions.a");
}

fn find_clang_include() -> PathBuf {
    let user_path = env::var("LIBCLANG_INCLUDE_PATH");
    let user_path = user_path.as_ref().map(|s|s.as_ref()).unwrap_or("./clang/include/");
    let candidate_paths = &[user_path,
        "/usr/lib/llvm-4.0/include/",
        "../clang/include/",
        "../../clang/include/",
        "../../../clang/include/",
        "../llvm/include/",
        "../../llvm/include/",
        "../../../llvm/include/",
    ];
    for &path in candidate_paths {
        let fspath = Path::new(path);
        if fspath.exists() && fspath.join("clang/AST").exists() {
            return fs::canonicalize(fspath).unwrap();
        }
        if let Some(parent) = fspath.parent() {
            println!("tried {}", parent.join("clang/AST").display());
            if parent.join("clang/AST").exists() {
                return fs::canonicalize(parent).unwrap();
            }
        }
    }

    println!("cargo:warning=Unable to find include/clang path. Set LIBCLANG_INCLUDE_PATH");
    return PathBuf::from("./clang/include/");
}
