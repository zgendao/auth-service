use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Uuid as UuidDiesel;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::str::FromStr;
use uuid;

#[derive(
    Clone, Debug, AsExpression, PartialEq, FromSqlRow, Serialize, Deserialize, Hash, Eq, Copy,
)]
#[sql_type = "UuidDiesel"]
pub struct Uuid(uuid::Uuid);

impl Uuid {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl ToSql<UuidDiesel, Pg> for Uuid {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.0.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<UuidDiesel, Pg> for Uuid {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        Ok(Uuid(uuid::Uuid::from_slice(bytes)?))
    }
}

impl<T> From<T> for Uuid
    where
        T: AsRef<str> + Sized,
{
    fn from(uuid: T) -> Self {
        Self(uuid::Uuid::from_str(uuid.as_ref()).unwrap())
    }
}
