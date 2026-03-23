load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

rust_library(
    name = "spider_rs_demo",
    srcs = ["src/lib.rs"],
    deps = [
        "@crate_index//:spider",
        "@crate_index//:reqwest",
        "@crate_index//:scraper",
        "@crate_index//:tokio",
        "@crate_index//:serde",
        "@crate_index//:serde_json",
        "@crate_index//:url",
        "@crate_index//:thiserror",
        "@crate_index//:log",
    ],
)

rust_binary(
    name = "spider_rs_demo_bin",
    srcs = ["src/main.rs"],
    deps = [
        ":spider_rs_demo",
        "@crate_index//:clap",
    ],
    edition = "2021",
)

rust_test(
    name = "spider_rs_demo_test",
    srcs = ["src/lib.rs"],
    deps = [
        ":spider_rs_demo",
    ],
    edition = "2021",
)
