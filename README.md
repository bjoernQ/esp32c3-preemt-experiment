# Experiment for preemtive multitasking on ESP32C3 in Rust

This is just a small experiment having a main task and two additional tasks running on ESP32C3.

It directly uses the PAC and is meant to be flashed in direct-boot mode.

Needs a nightly Rust (e.g _rustc 1.59.0-nightly (f1ce0e6a0 2022-01-05)_) compiler to build.

The code is meant to be as simple as possible to make it easy to understand what is happening.
