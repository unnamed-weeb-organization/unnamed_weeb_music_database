pub mod artist;
pub mod release;
pub mod song;

use juniper::{GraphQLEnum, GraphQLObject};
use sqlx::{Decode, Database, database::HasValueRef};

#[derive(GraphQLObject, Clone, Debug, sqlx::Type)]
pub struct Name {
    /// Native name the original variant uses.
    ///
    /// "残酷な天使のテーゼ"
    native: String,
    /// Romanized variant of the native title.
    ///
    /// "Zankoku na Tenshi no Tēze"
    romanized: String,
    /// English translated name.
    ///
    /// "The Cruel Angel's Thesis"
    english: String,
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
