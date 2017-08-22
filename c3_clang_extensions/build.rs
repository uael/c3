use std::process;
use std::env;
use std::path::{PathBuf, Path};
use std::fs;

extern crate gcc;

fn query_llvm_config(arg: &str) -> String {
    let llvm_config_cmd = env::var("LLVM_CONFIG_PATH").unwrap_or("llvm-config".to_string());
    let cmd_out = process::Command::new(llvm_config_cmd).arg(arg)
        .output()
        .expect("llvm-config must be in PATH or set LLVM_CONFIG_PATH to the llvm-config binary");
    String::from_utf8(cmd_out.stdout).expect("utf8")
        .trim_right().to_owned()
}

fn canonicalize<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = fs::canonicalize(path.as_ref()).unwrap();
    // MSVC sucks
    {let pathstr = path.to_str().unwrap();
    if pathstr.starts_with("\\\\?\\") {
        return PathBuf::from(&pathstr["\\\\?\\".len()..])
    }}
    path
}

fn main() {
    let inc_path = query_llvm_config("--includedir");
    let lib_inc_path = format!("{}/../include", query_llvm_config("--libdir"));
    let paths = [inc_path.as_str(), lib_inc_path.as_str()];

    let mut cfg = gcc::Config::new();
    cfg
                .cpp(true)
                .file("src/extensions.cpp")
                .include(&inc_path)
                .include(find_clang_include(paths.as_ref(), "clang/AST/OperationKinds.h"))
                .include(find_clang_include(paths.as_ref(), "llvm/Support/DataTypes.h"))
                .include(canonicalize("vendor"));
    if !env::var("TARGET").unwrap().contains("msvc") {
        cfg.flag("-std=c++11").flag("-Wno-comment");
    }
    cfg
                .compile("libc3_clang_extensions.a");
}

fn find_clang_include(llvm_paths: &[&str], file_search: &str) -> PathBuf {
    let user_paths = env::var("LIBCLANG_INCLUDE_PATH");
    let user_paths = user_paths.as_ref().map(|s|s.as_ref()).unwrap_or("./clang/include/");
    let candidate_paths: Vec<_> = env::split_paths(user_paths).collect();
    let fallback_paths = [
        llvm_paths[0],
        llvm_paths[1],
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
            return canonicalize(fspath);
        }
        if let Some(parent) = fspath.parent() {
            if parent.join(file_search).exists() {
                return canonicalize(parent);
            }
        }
    }

    println!("cargo:warning=Unable to find include path for '{}'. Set LIBCLANG_INCLUDE_PATH", file_search);
    return PathBuf::from("./clang/include/");
}
