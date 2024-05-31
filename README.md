# Stream Guard

[![crates.io](https://img.shields.io/crates/v/stream-guard)](https://crates.io/crates/stream-guard)
[![Build](https://github.com/fwcd/stream-guard/actions/workflows/build.yml/badge.svg)](https://github.com/fwcd/stream-guard/actions/workflows/build.yml)

A small RAII wrapper around a [`Stream`](https://docs.rs/futures/latest/futures/stream/trait.Stream.html) (asynchronous iterator) that automatically invokes a user-defined action upon being dropped.
