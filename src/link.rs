//! Pagination link.

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Link {
    pub href: String,
    #[serde(
        default = "default_templated_as_false",
        skip_serializing_if = "templated_is_false"
    )]
    pub templated: bool,
}

fn default_templated_as_false() -> bool {
    false
}

fn templated_is_false(v: &bool) -> bool {
    !v
}
