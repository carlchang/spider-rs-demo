load("@rules_rust//rust:repositories.bzl", "rust_register_toolchains", "rust_toolchain")

rust_toolchain(
    name = "rust",
    edition = "2021",
)
