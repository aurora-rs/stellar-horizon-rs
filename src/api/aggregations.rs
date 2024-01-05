use crate::api::assets::credit_asset_to_string;
use crate::error::{Error, Result};
use crate::page::Page;
use crate::request::{Order, Request, StreamRequest, UrlPageRequestExt};
use crate::resources;
use chrono::{DateTime, Duration, Utc};
use std::convert::TryInto;
use stellar_base::amount::{Amount, Stroops};
use stellar_base::asset::{Asset, CreditAsset};
use stellar_base::crypto::PublicKey;
use stellar_base::error::Error as StellarBaseError;
use url::Url;

/// Creates a request to retrieve order book data.
pub fn order_book(selling: Asset, buying: Asset) -> OrderBookRequest {
    OrderBookRequest {
        buying,
        selling,
        limit: None,
    }
}

/// Creates a request to retrieve information about potential path payments.
///
/// The strict receive payment path endpoint lists the paths a payment
/// can take based on the amount of an asset you want the recipient to
/// receive. The destination asset amount stays constant, and the type
/// and amount of an asset sent varies based on offers in the order
/// books.
pub fn paths_strict_receive<S: TryInto<Stroops>>(
    source_assets: Vec<CreditAsset>,
    destination_asset: Asset,
    destination_amount: S,
) -> Result<PathsStrictReceiveRequest> {
    let destination_amount = destination_amount
        .try_into()
        .map_err(|_| Error::StellarBaseError(StellarBaseError::InvalidStroopsAmount))?;
    Ok(PathsStrictReceiveRequest {
        source_account: None,
        source_assets,
        destination_asset,
        destination_amount,
    })
}

/// Creates a request to retrieve information about potential path payments.
///
/// The strict receive payment path endpoint lists the paths a payment
/// can take based on the amount of an asset you want to send. The
/// source asset amount stays constant, and the type and amount of an
/// asset received varies based on offers in the order books.
pub fn paths_strict_send<S: TryInto<Stroops>>(
    source_asset: Asset,
    destination_assets: Vec<CreditAsset>,
    source_amount: S,
) -> Result<PathsStrictSendRequest> {
    let source_amount = source_amount
        .try_into()
        .map_err(|_| Error::StellarBaseError(StellarBaseError::InvalidStroopsAmount))?;
    Ok(PathsStrictSendRequest {
        source_asset,
        source_amount,
        destination_assets,
        destination_account: None,
    })
}

/// Creates a request to retrieve aggregated trade data.
///
/// A trade aggregation represents aggregated statistics on an asset
/// pair (base and counter) for a specific time period. Trade
/// aggregations are useful to developers of trading clients and
/// provide historical trade data.
///
/// This endpoint displays trade data based on filters set in the arguments.
///
/// This is done by dividing a given time range into segments and
/// aggregating statistics, for a given asset pair (base, counter)
/// over each of these segments.
///
/// The duration of the segments is specified with the `resolution`
/// parameter. The start and end of the time range are given by
/// `start_time` and `end_time` respectively, which are both rounded
/// to the nearest multiple of `resolution` since epoch.
///
/// The individual segments are also aligned with multiples of
/// `resolution` since epoch. If you want to change this alignment, the
/// segments can be `offset` by specifying the offset parameter.
pub fn all_trades(
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    resolution: Resolution,
    base_asset: Asset,
    counter_asset: Asset,
) -> AllTradesRequest {
    AllTradesRequest {
        start_time,
        end_time,
        resolution,
        base_asset,
        counter_asset,
        offset: None,
        order: None,
        limit: None,
    }
}

/// Creates a request to retrieve statistics about network fees.
pub fn fee_stats() -> FeeStatsRequest {
    FeeStatsRequest {}
}

/// Trade aggregation resolution.
#[derive(Debug, Copy, Clone)]
pub enum Resolution {
    /// 1 minute.
    OneMinute,
    /// 5 minutes.
    FiveMinutes,
    /// 15 minutes.
    FifteenMinutes,
    /// 1 hour.
    OneHour,
    /// 1 day.
    OneDay,
    /// 1 week.
    OneWeek,
    /// Custom duration.
    ///
    /// Horizon only supports the value defined in this enum, a custom
    /// duration can be used as escape hatch if horizon introduces new
    /// values to this enum.
    Custom(Duration),
}

/// Request order book data.
#[derive(Debug, Clone)]
pub struct OrderBookRequest {
    limit: Option<u64>,
    selling: Asset,
    buying: Asset,
}

/// Request paths for path payment strict receive.
#[derive(Debug, Clone)]
pub struct PathsStrictReceiveRequest {
    source_account: Option<String>,
    source_assets: Vec<CreditAsset>,
    destination_asset: Asset,
    destination_amount: Stroops,
}

/// Request paths for path payment strict send.
#[derive(Debug, Clone)]
pub struct PathsStrictSendRequest {
    destination_account: Option<String>,
    source_asset: Asset,
    destination_assets: Vec<CreditAsset>,
    source_amount: Stroops,
}

/// Request aggregated trade data.
#[derive(Debug, Clone)]
pub struct AllTradesRequest {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    resolution: Resolution,
    base_asset: Asset,
    counter_asset: Asset,
    order: Option<Order>,
    limit: Option<i64>,
    offset: Option<Duration>,
}

/// Request fee stats.
#[derive(Debug, Clone)]
pub struct FeeStatsRequest {}

impl OrderBookRequest {
    /// The total number of records returned.
    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl PathsStrictReceiveRequest {
    /// Update the request to include only paths that `source_account` holds.
    pub fn with_source_account(mut self, source_account: &PublicKey) -> Self {
        self.source_account = Some(source_account.account_id());
        self
    }
}

impl PathsStrictSendRequest {
    /// Update the request to include only paths that `destination_acconut` can hold.
    pub fn with_destination_account(mut self, destination_account: &PublicKey) -> Self {
        self.destination_account = Some(destination_account.account_id());
        self
    }
}

impl Request for OrderBookRequest {
    type Response = resources::OrderBookSummary;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&["order_book"]);
        }
        base_url = base_url.append_asset_params(&self.buying, Some("buying"));
        base_url = base_url.append_asset_params(&self.selling, Some("selling"));
        if let Some(limit) = &self.limit {
            base_url = base_url.append_query_param("limit", &limit.to_string());
        }
        Ok(base_url)
    }
}

impl Request for PathsStrictReceiveRequest {
    type Response = Page<resources::Path>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&["paths", "strict-receive"]);
        }
        if let Some(source_account) = &self.source_account {
            base_url = base_url.append_query_param("source_account", source_account);
        }
        if !self.source_assets.is_empty() {
            let source_assets = serialize_assets_to_query_value(&self.source_assets);
            base_url = base_url.append_query_param("source_assets", &source_assets);
        }
        base_url = base_url.append_asset_params(&self.destination_asset, Some("destination"));
        let amount = Amount::from_stroops(&self.destination_amount)?;
        base_url = base_url.append_query_param("destination_amount", &amount.to_string());
        Ok(base_url)
    }
}

impl Request for PathsStrictSendRequest {
    type Response = Page<resources::Path>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&["paths", "strict-send"]);
        }
        if let Some(destination_account) = &self.destination_account {
            base_url = base_url.append_query_param("destination_account", destination_account);
        }
        if !self.destination_assets.is_empty() {
            let destination_assets = serialize_assets_to_query_value(&self.destination_assets);
            base_url = base_url.append_query_param("destination_assets", &destination_assets);
        }
        base_url = base_url.append_asset_params(&self.source_asset, Some("source"));
        let amount = Amount::from_stroops(&self.source_amount)?;
        base_url = base_url.append_query_param("source_amount", &amount.to_string());
        Ok(base_url)
    }
}

impl StreamRequest for OrderBookRequest {
    type Resource = resources::OrderBookSummary;
}

impl Request for AllTradesRequest {
    type Response = Page<resources::TradeAggregation>;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&["trade_aggregations"]);
        }

        let start_time = self.start_time.timestamp_millis();
        base_url = base_url.append_query_param("start_time", &start_time.to_string());
        let end_time = self.end_time.timestamp_millis();
        base_url = base_url.append_query_param("end_time", &end_time.to_string());
        let resolution = resolution_to_milliseconds(&self.resolution);
        base_url = base_url.append_query_param("resolution", &resolution.to_string());
        if let Some(offset) = &self.offset {
            base_url =
                base_url.append_query_param("offset", &offset.num_milliseconds().to_string());
        }
        base_url = base_url.append_asset_params(&self.base_asset, Some("base"));
        base_url = base_url.append_asset_params(&self.counter_asset, Some("counter"));
        if let Some(order) = &self.order {
            base_url = base_url.append_query_param("order", &order.to_query_value());
        }
        if let Some(limit) = &self.limit {
            base_url = base_url.append_query_param("limit", &limit.to_string());
        }
        Ok(base_url)
    }
}

impl Request for FeeStatsRequest {
    type Response = resources::FeeStats;

    fn uri(&self, base_url: &Url) -> Result<Url> {
        let mut base_url = base_url.clone();
        {
            let mut segments = base_url.path_segments_mut().unwrap();
            segments.extend(&["fee_stats"]);
        }
        Ok(base_url)
    }
}

fn serialize_assets_to_query_value(assets: &[CreditAsset]) -> String {
    let assets: Vec<_> = assets.iter().map(credit_asset_to_string).collect();
    assets.join(",")
}

fn resolution_to_milliseconds(resolution: &Resolution) -> u64 {
    match resolution {
        Resolution::OneMinute => 60000,
        Resolution::FiveMinutes => 300000,
        Resolution::FifteenMinutes => 900000,
        Resolution::OneHour => 3600000,
        Resolution::OneDay => 86400000,
        Resolution::OneWeek => 604800000,
        Resolution::Custom(d) => d.num_milliseconds() as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::Request;
    use std::collections::HashMap;
    use stellar_base::asset::Asset;
    use stellar_base::crypto::PublicKey;
    use url::Url;

    fn host() -> Url {
        "https://horizon.stellar.org".parse().unwrap()
    }

    fn base_url() -> Url {
        "https://horizon.stellar.org/some/non/host/url"
            .parse()
            .unwrap()
    }

    fn keypair0() -> PublicKey {
        PublicKey::from_account_id("GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623")
            .unwrap()
    }

    fn credit_asset0() -> Asset {
        let issuer = keypair0();
        let code = "ABCD";
        Asset::new_credit(code, issuer).unwrap()
    }

    #[test]
    fn test_order_book_request_uri() {
        let req = order_book(credit_asset0(), Asset::new_native());
        let uri = req.uri(&host()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/order_book?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"native".to_string()), query.get("buying_asset_type"));
        assert_eq!(None, query.get("buying_asset_code"));
        assert_eq!(None, query.get("buying_asset_issuer"));
        assert_eq!(
            Some(&"credit_alphanum4".to_string()),
            query.get("selling_asset_type")
        );
        assert_eq!(Some(&"ABCD".to_string()), query.get("selling_asset_code"));
        assert_eq!(
            Some(&"GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623".to_string()),
            query.get("selling_asset_issuer")
        );
        assert_eq!(None, query.get("limit"));
    }

    #[test]
    fn test_order_book_request_uri_with_base_url() {
        let req = order_book(credit_asset0(), Asset::new_native());
        let uri = req.uri(&&base_url()).unwrap();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/order_book?"));
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert_eq!(Some(&"native".to_string()), query.get("buying_asset_type"));
        assert_eq!(None, query.get("buying_asset_code"));
        assert_eq!(None, query.get("buying_asset_issuer"));
        assert_eq!(
            Some(&"credit_alphanum4".to_string()),
            query.get("selling_asset_type")
        );
        assert_eq!(Some(&"ABCD".to_string()), query.get("selling_asset_code"));
        assert_eq!(
            Some(&"GDHCYXWSMCGPN7S5VBCSDVNXUMRI62MCRVK7DBULCDBBIEQE76DND623".to_string()),
            query.get("selling_asset_issuer")
        );
        assert_eq!(None, query.get("limit"));
    }

    #[test]
    fn test_order_book_request_uri_with_limit() {
        let req = order_book(credit_asset0(), Asset::new_native()).with_limit(100);
        let uri = req.uri(&host()).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/order_book?"));
        assert_eq!(Some(&"100".to_string()), query.get("limit"));
    }

    #[test]
    fn test_order_book_request_uri_with_limit_with_base_url() {
        let req = order_book(credit_asset0(), Asset::new_native()).with_limit(100);
        let uri = req.uri(&base_url()).unwrap();
        let query: HashMap<_, _> = uri.query_pairs().into_owned().collect();
        assert!(uri
            .to_string()
            .starts_with("https://horizon.stellar.org/some/non/host/url/order_book?"));
        assert_eq!(Some(&"100".to_string()), query.get("limit"));
    }
}
