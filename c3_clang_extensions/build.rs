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
                .include(find_clang_include(path, "clang/AST/OperationKinds.h"))
                .include(find_clang_include(path, "llvm/Support/DataTypes.h"))
                .include(fs::canonicalize("vendor").unwrap())
                .compile("libc3_clang_extensions.a");
}

fn find_clang_include(llvm_path: &str, file_search: &str) -> PathBuf {
    let user_paths = env::var("LIBCLANG_INCLUDE_PATH");
    let user_paths = user_paths.as_ref().map(|s|s.as_ref()).unwrap_or("./clang/include/");
    let candidate_paths: Vec<_> = env::split_paths(user_paths).collect();
    let fallback_paths = [
        llvm_path,
        "/usr/lib/llvm-4.0/include/",
        "../clang/include/",
        "../../clang/include/",
        "../../../clang/include/",
        "../llvm/include/",
        "../../llvm/include/",
        "../../../llvm/include/",
    ];
    let candidate_paths = candidate_paths.iter()
        .map(|p|p.as_path())
        .chain(fallback_paths.into_iter().map(Path::new));

    for fspath in candidate_paths {
        if fspath.exists() && fspath.join(file_search).exists() {
            return fs::canonicalize(fspath).unwrap();
        }
        if let Some(parent) = fspath.parent() {
            if parent.join(file_search).exists() {
                return fs::canonicalize(parent).unwrap();
            }
        }
    }

    println!("cargo:warning=Unable to find include path for '{}'. Set LIBCLANG_INCLUDE_PATH", file_search);
    return PathBuf::from("./clang/include/");
}
