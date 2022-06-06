use super::{artist::Artist, ExternalSites, Name};
use juniper::{GraphQLEnum, GraphQLObject};

#[derive(GraphQLEnum, Clone, Debug)]
pub enum ReleaseType {
    Album,
    Single,
    EP,
}

#[derive(GraphQLObject, Clone, Debug)]
/// Release done by one or multiple artist
///
/// This structure simply represents an album but has a fancy name to not to
/// confuse it with [`ReleaseType::Album`]
pub struct Release {
    id: String,
    name: Name,
    release_type: ReleaseType,
    release_artists: Vec<Artist>,
    total_tracks: i32,
    /// Contains an array of external links (YouTube, Apple Music and etc)
    external_sites: Vec<ExternalSites>,
}
