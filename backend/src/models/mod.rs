pub mod artist;
pub mod release;
pub mod song;

use std::error::Error;

use juniper::{GraphQLEnum, GraphQLObject};
use sea_query::enum_def;

#[derive(Clone, Debug, juniper::GraphQLInputObject)]
pub struct NewName {
    pub native: String,
    pub romanized: String,
    pub english: String,
}

#[derive(GraphQLObject, Clone, Debug, sqlx::Encode)]
#[enum_def]
pub struct Name {
    /// Native name the original variant uses.
    ///
    /// "残酷な天使のテーゼ"
    pub native: String,
    /// Romanized variant of the native title.
    ///
    /// "Zankoku na Tenshi no Tēze"
    pub romanized: String,
    /// English translated name.
    ///
    /// "The Cruel Angel's Thesis"
    pub english: String,
}

#[derive(GraphQLEnum, Clone, Debug)]
pub enum ExternalSite {
    AppleMusic,
    YouTube,
    Spotify,
}

#[derive(GraphQLObject, Clone, Debug)]
pub struct ExternalSites {
    site_type: ExternalSite,
    url: String,
}

impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for Name {
    fn decode(
        value: sqlx::postgres::PgValueRef<'r>,
    ) -> Result<Self, Box<dyn Error + 'static + Send + Sync>> {
        let mut decoder = sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let native = decoder.try_decode::<String>()?;
        let romanized = decoder.try_decode::<String>()?;
        let english = decoder.try_decode::<String>()?;
        Ok(Name {
            native,
            romanized,
            english,
        })
    }
}

impl sqlx::Type<sqlx::Postgres> for Name {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("localized_name")
    }
}