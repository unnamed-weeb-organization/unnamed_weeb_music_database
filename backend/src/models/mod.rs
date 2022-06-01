pub mod artist;
pub mod release;
pub mod song;

use juniper::{GraphQLEnum, GraphQLObject};

#[derive(GraphQLObject)]
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
    english: Option<String>,
}

#[derive(GraphQLEnum)]
pub enum ExternalSite {
    AppleMusic,
    YouTube,
    Spotify,
}

#[derive(GraphQLObject)]
pub struct ExternalSites {
    site_type: ExternalSite,
    url: String,
}
