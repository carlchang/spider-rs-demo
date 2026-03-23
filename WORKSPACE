workspace(name = "spider_rs_demo")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "634f2628e8b4e9e8f1c8f1c8f1c8f1c8f1c8f1c8f1c8f1c8f1c8f1c8f1c8f1c8f",
    strip_prefix = "rules_rust-0.48.0",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.48.0/rules_rust-0.48.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rust_register_toolchains", "rust_toolchain")

rust_register_toolchains(
    version = "1.75.0",
    edition = "2021",
)

rust_toolchain(
    name = "rust",
    edition = "2021",
)
