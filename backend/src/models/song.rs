use crate::utils::context::Context;

use super::{artist::Artist, release::Release, Name};
use juniper::graphql_object;
use ulid::Ulid;

#[derive(Clone, Debug)]
pub struct Song {
    pub id: Ulid,
    pub name: Name
    /*/// Contains an array of external links (YouTube, Apple Music and etc)
    pub external_sites: Vec<ExternalSites>,*/
}

#[graphql_object(Context = Context)]
impl Song {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> &Name {
        &self.name
    }

    async fn artists(&self, context: &Context) -> Vec<Artist> {
        let db = &*context.db;
        Artist::get_artists_by_song_id(&self.id, db).await.unwrap()
    }

    async fn releases(&self, context: &Context) -> Vec<Release> {
        let db = &*context.db;
        Release::get_releases_by_song_id(&self.id, db).await.unwrap()
    }
}