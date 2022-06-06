use super::{artist::Artist, release::Release, ExternalSites, Name};
use juniper::GraphQLObject;

#[derive(GraphQLObject, Clone, Debug)]
pub struct Song {
    pub id: String,
    pub name: Name,
    /*pub artists: Vec<Artist>,
    pub releases: Vec<Release>,
    /// Contains an array of external links (YouTube, Apple Music and etc)
    pub external_sites: Vec<ExternalSites>,*/
}
