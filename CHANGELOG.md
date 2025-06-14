# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Add `Payment::base()`.
- Add `HorizonHttpClient::with_extra_headers` to support authorization headers
- Add `Operation::InvokeHostFunction`
- Add `Operation::ExtendFootprintTTL`
- Add `Operation::RestoreFootprint`
- Add `Payment::InvokeHostFunction`

## [0.7.1] - 2023-11-08
### Changed
- Remove 'sodium\_oxide' feature from the 'stellar-base' dependency, which is
  heavy weight and only needed for signing.

## [0.7.0] - 2023-11-03
### Added
- Add liquidity pool endpoints
- Add claimable balance related endpoints
- Add `LiquidityPool`, `LiquidityPoolLinks`, `LiquidityPoolReserve`, `AssetAmount`, `TradePrice`
and `LiquidityPoolOrAsset` resources.
- Add `LiquidityPoolDepositedEffect`
- Add `LiquidityPoolWithdrewEffect`
- Add `LiquidityPoolTradeEffect`
- Add `LiquidityPoolCreatedEffect`
- Add `LiquidityPoolRemovedEffect`
- Add `LiquidityPoolRevokedEffect`
- Add `ClaimableBalanceFlags` resource
- Add `Clawback`, `ClawbackClaimableBalance`, `SetTrustLineFlags`, `LiquidityPoolDeposit` and `LiquidityPoolWithdraw` operations
- Add `TradePrice` resource

### Changed
- Update `ChangeTrustOperation`
- Update `Trade` resource
- Update `AccountFlags` resource
- Update `ClaimableBalance` resource
- Update `Root` resource
- Update `Transaction` resource
- Update most effects to support liquidity pools
- Update most effects to support muxed accounts
- Update most operations to support muxed accounts
- Update operations to support liquidity pools
- Change deserialization for `Predicate` to support the extra `abs_before_epoch` field
- Update dependencies
- BREAKING CHANGE: Update account's asset `Balance` resource
- BREAKING CHANGE: Update `AssetStat` resource
- BREAKING CHANGE: Update `Trade` resource
- BREAKING CHANGE: Update `TradeAggregation` resource


## [0.6.0] - 2020-01-20
### Added
 - Add claimable balances endpoints
 - Add `CreateClaimableBalanceOperation`
 - Add `ClaimClaimableBalanceOperation`
 - Add `BeginSponsoringFutureReserversOperation`
 - Add `EndSponsoringFutureReserversOperation`
 - Add `RevokeSponsorshipOperation`
 - Add `ClaimableBalanceCreatedEffect`
 - Add `ClaimableBalanceClaimantCreatedEffect`
 - Add `ClaimableBalanceClaimedEffect`
 - Add `AccountSponsorshipCreatedEffect`
 - Add `AccountSponsorshipUpdatedEffect`
 - Add `AccountSponsorshipRemovedEffect`
 - Add `TrustLineSponsorshipCreatedEffect`
 - Add `TrustLineSponsorshipUpdatedEffect`
 - Add `TrustLineSponsorshipRemovedEffect`
 - Add `ClaimableBalanceSponsorshipCreatedEffect`
 - Add `ClaimableBalanceSponsorshipUpdatedEffect`
 - Add `ClaimableBalanceSponsorshipRemovedEffect`
 - Add `SignerSponsorshipCreatedEffect`
 - Add `SignerSponsorshipUpdatedEffect`
 - Add `SignerSponsorshipRemovedEffect`
 - Add `ClaimableBalance`, `Claimant`, and `Predicate` resources

### Changed
 - Update `stellar-base` dependency
 - Update tokio-related dependencies
 - Add sponsor filter to the accounts endpoint
 - Add sponsor filter to the offers endpoint
 - Update `Account` resource with sponsor
 - Update `Balance` resource with sponsor
 - Update `Signer` resource with sponsor
 - Update `AccountData` resource with sponsor
 - Update `HorizonError` to handle all error responses


## [0.5.0] - 2020-07-30
### Added
 - Add `/accounts` api endpoint

### Changed
 - Api now accepts `S: Into<String>`
 - Parse offers and ledgers ids
 - Standardise ledger id type


## [0.4.3] - 2020-07-24
### Changed
 - `TransactionsForAccountRequest` implements `StreamRequest`
 - `TransactionsForLedgerRequest` implements `StreamRequest`

## [0.4.2] - 2020-07-24
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
