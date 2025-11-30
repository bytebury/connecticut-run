use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Town {
    pub id: i64,
    pub name: String,
    pub display_name: String,
}

impl Town {
    pub fn is_valid(town_id: i64) -> bool {
        (1..169).contains(&town_id)
    }

    pub fn is_not_valid(town_id: i64) -> bool {
        !Self::is_valid(town_id)
    }
}

impl fmt::Display for Town {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display_name)
    }
}
