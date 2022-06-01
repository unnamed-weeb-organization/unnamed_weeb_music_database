pub enum ArtistType {
    Unknown,
    Singer,
    Producer,
    Remixer,
}

pub struct Artist {
    id: String,
    name: String,
    sort_name: String,
    artist_type: ArtistType,
    external_sites: Vec<String>,
}

pub enum ReleaseType {
    Album,
    Single,
    EP,
}

pub struct Release {
    id: String,
    name: String,
    sort_name: String,
    release_type: ReleaseType,
    release_artists: Vec<String>,
    total_tracks: i32,
    external_sites: Vec<String>,
}

pub struct Song {
    id: String,
    name: String,
    sort_name: String,
    artists: Vec<String>,
    releases: Vec<String>,
    external_sites: Vec<String>,
}
