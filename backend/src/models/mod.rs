use juniper::{GraphQLEnum, GraphQLObject};

// #[derive(GraphQLEnum)]
// pub enum ArtistType {
//     Unknown,
//     Singer,
//     Producer,
//     Remixer,
// }

#[derive(GraphQLObject)]
pub struct Artist {
    id: String,
    name: String,
    sort_name: String,
    /// Contains an array of external links (YouTube, Apple Music and etc)
    external_sites: Vec<String>,
    // artist_type: ArtistType, <- TODO: Find a better way to handle artist types without trying to duplicate the source.
}

#[derive(GraphQLEnum)]
pub enum ReleaseType {
    Album,
    Single,
    EP,
}

#[derive(GraphQLObject)]
/// Release done by one or multiple artist
///
/// This structure simply represents an album but has a fancy name to not to
/// confuse it with [`ReleaseType::Album`]
pub struct Release {
    id: String,
    name: String,
    sort_name: String,
    release_type: ReleaseType,
    release_artists: Vec<String>,
    total_tracks: i32,
    /// Contains an array of external links (YouTube, Apple Music and etc)
    external_sites: Vec<String>,
}

#[derive(GraphQLObject, Default)]
pub struct Song {
    id: String,
    name: String,
    sort_name: String,
    artists: Vec<String>,
    releases: Vec<String>,
    /// Contains an array of external links (YouTube, Apple Music and etc)
    external_sites: Vec<String>,
}
