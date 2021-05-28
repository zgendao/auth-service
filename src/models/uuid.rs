use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Uuid as UuidDiesel;
use serde::{Deserialize, Serialize};
use std::io::Write;
use uuid;
use std::str::FromStr;

#[derive(
    Clone, Debug, AsExpression, PartialEq, FromSqlRow, Serialize, Deserialize, Hash, Eq, Copy,
)]
#[sql_type = "UuidDiesel"]
pub struct Uuid(pub uuid::Uuid);

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

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Uuid(uuid)
    }
}

impl From<String> for Uuid {
    fn from(uuid: String) -> Self {
        Uuid(uuid::Uuid::from_str(&*uuid).unwrap())
    }
}
