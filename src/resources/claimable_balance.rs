use crate::error::{Error, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use stellar_base::claim::ClaimPredicate;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ClaimableBalance {
    pub id: String,
    pub paging_token: String,
    pub asset: String,
    pub amount: String,
    pub sponsor: Option<String>,
    pub last_modified_ledger: i64,
    pub claimants: Vec<Claimant>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Claimant {
    pub destination: String,
    pub predicate: Predicate,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Predicate {
    And(Vec<Box<Predicate>>),
    Or(Vec<Box<Predicate>>),
    Not(Box<Predicate>),
    Unconditional(bool),
    AbsBefore(DateTime<Utc>),
    RelBefore(i64),
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
            Predicate::RelBefore(seconds) => {
                let duration = Duration::seconds(*seconds);
                Ok(ClaimPredicate::new_before_relative_time(duration))
            }
            Predicate::AbsBefore(datetime) => {
                Ok(ClaimPredicate::new_before_absolute_time(datetime.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Predicate;

    #[test]
    fn test_claim_predicate_serde() {
        let json = r#"{"and":[{"or":[{"rel_before":12},{"abs_before":"2020-08-26T11:15:39Z"}]},{"not":{"unconditional":true}}]}"#;
        let predicate: Predicate = serde_json::from_str(json).unwrap();

        let _claim_predicate = predicate.to_claim_predicate().unwrap();
    }
}
