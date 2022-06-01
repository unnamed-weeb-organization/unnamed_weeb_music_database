use super::{artist::Artist, release::Release, ExternalSites, Name};
use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Song {
    id: String,
    name: Name,
    artists: Vec<Artist>,
    releases: Vec<Release>,
    /// Contains an array of external links (YouTube, Apple Music and etc)
    external_sites: Vec<ExternalSites>,
}
