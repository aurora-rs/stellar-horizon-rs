# Rust Stellar Horizon Client

![CI](https://github.com/aurora-rs/stellar-horizon-rs/workflows/CI/badge.svg)
[![version](https://img.shields.io/crates/v/stellar-horizon)](https://crates.io/crates/stellar-horizon)
[![codecov](https://codecov.io/gh/aurora-rs/stellar-horizon-rs/branch/master/graph/badge.svg?token=3DR7ZYCPTQ)](https://codecov.io/gh/aurora-rs/stellar-horizon-rs)
[![License](https://img.shields.io/crates/l/stellar-horizon)](https://github.com/aurora-rs/stellar-horizon-rs/blob/master/LICENSE)


## Introduction

This crate contains a client for Stellar Horizon, supporting both the
REST API and streaming mode.

If you are looking for a crate to build Stellar transactions, look at
[stellar-base](https://github.com/aurora-rs/stellar-base-rs).


## Need help? Do you want to provide feeback?

You can join the discussion on our mailing list at
[`~aurora/stellar-sdk@lists.sr.ht`](https://lists.sr.ht/~aurora/stellar-sdk).


## Features

 * Easy to mock for testing
 * Support all Horizon endpoints
 * Support for Horizon streaming mode
 * Completely async


## Documentation

You can find the documentation on [docs.rs](https://docs.rs/stellar-horizon).


## Roadmap

This crate is still a work in progress. Relevant SEPs will be
implemented as separate crates when possible.

 - [x] Request single resources
 - [x] Request paginated resources
 - [x] Stream resources
 - [ ] Cleanup API, e.g. turn `String` into `Into<String>`
 - [x] Submit transactions
 - [x] Expose rate limit headers
 - [ ] Lazily parse resources fields
 - [ ] Navigation support, follow links
 - [ ] Improve documentation
 - [ ] Link to example applications

## Changelog

[You can find a changelog here.](https://github.com/aurora-rs/stellar-horizon-rs/blob/master/CHANGELOG.md)
