use diesel::{AsExpression, FromSqlRow};

// Build a uuid class based on uuid::Uuid.
macro_rules! uuid_wrapper {
    ($name: ident) => {
        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, FromSqlRow, AsExpression)]
        #[sql_type = "diesel::pg::types::sql_types::Uuid"]
        pub struct $name(pub uuid::Uuid);

        impl From<uuid::Uuid> for $name {
            fn from(id: uuid::Uuid) -> Self {
                $name(id)
            }
        }

        impl std::ops::Deref for $name {
            type Target = uuid::Uuid;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self ,serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer {
                    self.0.to_simple().to_string().serialize(serializer)
                }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where D: serde::Deserializer<'de> {
                uuid::Uuid::deserialize(deserializer).map($name)
            }
        }

        impl diesel::serialize::ToSql<diesel::sql_types::Uuid, diesel::pg::Pg> for $name {
            fn to_sql<W: std::io::Write>(&self, out: &mut diesel::serialize::Output<W, diesel::pg::Pg>) -> diesel::serialize::Result {
                <uuid::Uuid as diesel::serialize::ToSql<diesel::sql_types::Uuid, diesel::pg::Pg>>::to_sql(&self.0, out)
            }
        }

        impl<A> diesel::deserialize::FromSql<A, diesel::pg::Pg> for $name {
            fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
                uuid::Uuid::from_sql(bytes).map($name)
            }
        }

          impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0.to_simple())
            }
          }

        impl $name {
            #[allow(unused)]
            pub fn into_uuid(self) -> uuid::Uuid {
                self.0
            }
        }
    };
}

uuid_wrapper!(APITokenID);
uuid_wrapper!(PasteID);
uuid_wrapper!(UserID);
