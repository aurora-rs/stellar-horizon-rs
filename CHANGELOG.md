# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
 - Transactions api endpoints now support `include_failed` flag


## [0.4.1] - 2020-07-24
### Added
 - Add navigation links to `Page<T>`
 - Implement `Serialize` for `Page<T>`


## [0.4.0] - 2020-07-21
### Added
 - Parse response header to obtain rate limit information

### Changed
 - Request now returns a `(header, response)` tuple


## [0.3.0] - 2020-07-19
### Added
 - Implement all endpoints


## [0.2.3] - 2020-07-18
### Fixed
 - Mark stream response as `'static`


## [0.2.2] - 2020-07-18
### Fixed
 - Mark stream response as `Send`


## [0.2.1] - 2020-07-18
### Fixed
 - Make `HorizonHttpStream` implement `Send`


## [0.2.0] - 2020-07-18
### Added
 - Implement several endpoints
 - Improve documentation


### Changed
 - Update to `stellar-base` version `0.4.0`


## [0.1.0] - 2020-07-14
### Added
 - Horizon HTTP endpoints
 - Horizon streaming
