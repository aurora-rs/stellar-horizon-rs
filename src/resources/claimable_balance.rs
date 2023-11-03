use crate::error::{Error, Result};
use crate::link::Link;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use stellar_base::claim::ClaimPredicate;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalance {
    #[serde(rename = "_links")]
    pub links: ClaimableBalanceLinks,
    pub id: String,
    pub asset: String,
    pub amount: String,
    pub sponsor: Option<String>,
    pub last_modified_ledger: i64,
    pub last_modified_time: DateTime<Utc>,
    pub claimants: Vec<Claimant>,
    pub flags: ClaimableBalanceFlags,
    pub paging_token: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalanceLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub transactions: Link,
    pub operations: Link,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Claimant {
    pub destination: String,
    pub predicate: Predicate,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Predicate {
    And(Vec<Box<Predicate>>),
    Or(Vec<Box<Predicate>>),
    Not(Box<Predicate>),
    Unconditional(bool),
    AbsBefore(DateTime<Utc>),
    RelBefore(i64),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalanceFlags {
    pub clawback_enabled: bool,
}

impl Predicate {
    pub fn to_claim_predicate(&self) -> Result<ClaimPredicate> {
        match self {
            Predicate::And(inner) => {
                let mut predicates = inner.iter();
                match (predicates.next(), predicates.next()) {
                    (Some(p1), Some(p2)) => {
                        let p1_claim_predicate = p1.to_claim_predicate()?;
                        let p2_claim_predicate = p2.to_claim_predicate()?;
                        Ok(ClaimPredicate::new_and(
                            p1_claim_predicate,
                            p2_claim_predicate,
                        ))
                    }
                    _ => Err(Error::InvalidPredicate),
                }
            }
            Predicate::Or(inner) => {
                let mut predicates = inner.iter();
                match (predicates.next(), predicates.next()) {
                    (Some(p1), Some(p2)) => {
                        let p1_claim_predicate = p1.to_claim_predicate()?;
                        let p2_claim_predicate = p2.to_claim_predicate()?;
                        Ok(ClaimPredicate::new_or(
                            p1_claim_predicate,
                            p2_claim_predicate,
                        ))
                    }
                    _ => Err(Error::InvalidPredicate),
                }
            }
            Predicate::Not(inner) => {
                let inner_claim_predicate = inner.to_claim_predicate()?;
                Ok(ClaimPredicate::new_not(inner_claim_predicate))
            }
            Predicate::Unconditional(_) => Ok(ClaimPredicate::new_unconditional()),
            Predicate::AbsBefore(datetime) => {
                Ok(ClaimPredicate::new_before_absolute_time(*datetime))
            }
            Predicate::RelBefore(seconds) => {
                let duration = Duration::seconds(*seconds);
                Ok(ClaimPredicate::new_before_relative_time(duration))
            }
        }
    }
}

impl<'de> Deserialize<'de> for Predicate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_json::Value;

        let mut value = Value::deserialize(deserializer)?;

        if let Some(inner) = value.get_mut("and") {
            let inner_predicates: Vec<Box<Predicate>> =
                serde_json::from_value(inner.take()).map_err(serde::de::Error::custom)?;

            return Ok(Predicate::And(inner_predicates));
        } else if let Some(inner) = value.get_mut("or") {
            let inner_predicates: Vec<Box<Predicate>> =
                serde_json::from_value(inner.take()).map_err(serde::de::Error::custom)?;

            return Ok(Predicate::Or(inner_predicates));
        } else if let Some(inner) = value.get_mut("not") {
            let inner_predicate: Box<Predicate> =
                serde_json::from_value(inner.take()).map_err(serde::de::Error::custom)?;

            return Ok(Predicate::Not(inner_predicate));
        } else if let Some(inner) = value.get_mut("unconditional") {
            let p: bool = serde_json::from_value(inner.take()).map_err(serde::de::Error::custom)?;

            return Ok(Predicate::Unconditional(p));
        } else if let Some(inner) = value.get_mut("abs_before") {
            let p: DateTime<Utc> =
                serde_json::from_value(inner.take()).map_err(serde::de::Error::custom)?;

            return Ok(Predicate::AbsBefore(p));
        } else if let Some(inner) = value.get_mut("rel_before") {
            let p_str: String =
                serde_json::from_value(inner.take()).map_err(serde::de::Error::custom)?;
            let p = i64::from_str(&p_str).map_err(serde::de::Error::custom)?;

            return Ok(Predicate::RelBefore(p));
        }

        Err(serde::de::Error::custom("Invalid `Predicate` type"))
    }
}

#[cfg(test)]
mod tests {
    use super::Predicate;

    #[test]
    fn test_claim_predicate_serde() {
        let json = r#"{"and":[{"or":[{"rel_before":"12"},{"abs_before":"2020-08-26T11:15:39Z","abs_before_epoch": "1647448331"}]},{"not":{"unconditional":true}}]}"#;
        let predicate: Predicate = serde_json::from_str(json).unwrap();

        let _claim_predicate = predicate.to_claim_predicate().unwrap();
    }
}
