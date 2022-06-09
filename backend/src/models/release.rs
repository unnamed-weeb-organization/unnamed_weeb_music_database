use super::Name;
use juniper::GraphQLEnum;
use ulid::Ulid;

#[derive(GraphQLEnum, Clone, Debug, sqlx::Type)]
pub enum ReleaseType {
    Album,
    Single,
    EP,
}

#[derive(Clone, Debug)]
/// Release done by one or multiple artist
///
/// This structure simply represents an album but has a fancy name to not to
/// confuse it with [`ReleaseType::Album`]
pub struct Release {
    pub id: Ulid,
    pub name: Name,
    pub release_type: ReleaseType,
    pub total_tracks: i32,
}

#[graphql_object]
impl Release {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> &Name {
        &self.name
    }

    fn release_type(&self) -> &ReleaseType {
        &self.release_type
    }

    fn total_tracks(&self) -> &i32 {
        &self.total_tracks
    }

    /*fn artists(&self, context: &Context) -> Vec<Artist> {
        Artist::get_artists_by_release_id(&self.id, context.db).unwrap()
    }

    fn songs(&self, context: &Context) -> Vec<Song> {
        Song::get_songs_by_release_id(&self.id, context.db).unwrap()
    }*/
}