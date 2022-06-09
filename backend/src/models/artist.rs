use super::Name;
use ulid::Ulid;

// #[derive(GraphQLEnum)]
// pub enum ArtistType {
//     Unknown,
//     Singer,
//     Producer,
//     Remixer,
// }

#[derive(Clone, Debug)]
pub struct Artist {
    pub id: Ulid,
    pub name: Name,
    ///// Contains an array of external links (YouTube, Apple Music and etc)
    //pub external_sites: Option<Vec<ExternalSites>>,
    // artist_type: ArtistType, <- TODO: Find a better way to handle artist types without trying to duplicate the source.
}

#[graphql_object]
impl Artist {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> &Name {
        &self.name
    }
}