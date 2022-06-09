pub mod artist;
pub mod release;
pub mod song;

use juniper::{GraphQLEnum, GraphQLObject};


#[derive(GraphQLObject, Clone, Debug)]
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

#[automatically_derived]
    impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Name {
        fn decode(
            value: ::sqlx::postgres::PgValueRef<'r>,
        ) -> ::std::result::Result<
            Self,
            ::std::boxed::Box<
                dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
            >,
        > {
            let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
            let native = decoder.try_decode::<String>()?;
            let romanized = decoder.try_decode::<String>()?;
            let english = decoder.try_decode::<String>()?;
            ::std::result::Result::Ok(Name {
                native,
                romanized,
                english,
            })
        }
    }
    #[automatically_derived]
    impl ::sqlx::Type<::sqlx::Postgres> for Name {
        fn type_info() -> ::sqlx::postgres::PgTypeInfo {
            ::sqlx::postgres::PgTypeInfo::with_name("localized_name")
        }
    }