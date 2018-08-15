cmake_minimum_required(VERSION 3.12)
project(selenium_rs)

set(CMAKE_CXX_STANDARD 14)

add_executable(selenium_rs
        src/selenium_core/mod.rs
        src/selenium_core/url_builder.rs
        src/lib.rs
        src/macros.rs
        target/debug/build/openssl-sys-b59d60fd5a6e0865/out/expando.c
        tests/webdriver_tests.rs
        Cargo.lock
        Cargo.toml
        rusty-tags.vi
        selenium-rs.iml)
