use super::{Name, ExternalSites};
use juniper::GraphQLObject;
use sqlx::FromRow;

// #[derive(GraphQLEnum)]
// pub enum ArtistType {
//     Unknown,
//     Singer,
//     Producer,
//     Remixer,
// }

#[derive(GraphQLObject, Clone, Debug, FromRow)]
pub struct Artist {
    id: String,
    name: Name,
    /// Contains an array of external links (YouTube, Apple Music and etc)
    external_sites: Vec<ExternalSites>,
    // artist_type: ArtistType, <- TODO: Find a better way to handle artist types without trying to duplicate the source.
}
