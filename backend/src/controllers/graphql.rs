use crate::{models::{song::{Song, NewSong}, Name, artist::Artist}, utils::context::Context, controllers::page::{Page, PageInfo}};
use hyper::{Body, Request, Response};
use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest}, EmptySubscription, FieldResult, RootNode, graphql_object, LookAheadMethods,
};
use routerify::prelude::*;
use sqlx::PgPool;
use std::{io, sync::Arc};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;
type LocalSchema = Arc<Schema>;

pub async fn graphiql(_: Request<Body>) -> Result<Response<Body>, io::Error> {
    let html = graphiql_source("/graphql", None);
    Ok(Response::new(Body::from(html)))
}

pub async fn graphql(req: Request<Body>) -> Result<Response<Body>, io::Error> {
    let data = req.data::<LocalSchema>().unwrap().clone();
    let db = req.data::<PgPool>().unwrap().clone();
	let ctx = Context::new(db.clone());
    let request = deserialize_body(req.into_body()).await?;
    let response = request.execute(&data, &ctx).await;

    Ok(Response::new(Body::from(
        serde_json::to_string(&response).unwrap(),
    )))
}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    async fn Song<'ctx>(id: Option<String>, search: Option<String>, artist_id: Option<String>, release_id: Option<String>, genres: Option<Vec<String>>, context: &'ctx Context) -> FieldResult<Song> {
        // Ok(Song)
        let db = &*context.db;
        let mut options = crate::models::song::Options {
            id: None,
            search: None,
            artist_id: None,
            release_id: None,
            genres: None,
            page: None,
            per_page: None,
        };

        if let Some(id) = id {
            options.id = Some(id);
        }

        if let Some(search) = search {
            options.search = Some(search);
        }

        if let Some(artist_id) = artist_id {
            options.artist_id = Some(artist_id);
        }

        if let Some(release_id) = release_id {
            options.release_id = Some(release_id);
        }

        if let Some(genres) = genres {
            options.genres = Some(genres);
        }

        Ok(crate::database::song::get_song(&options, db).await.unwrap())
    }

    async fn Artist<'ctx>(id: Option<String>, search: Option<String>, song_id: Option<String>, release_id: Option<String>, context: &'ctx Context) -> FieldResult<Artist> {
        // Ok(Artist)
        let db = &*context.db;
        let mut options = crate::models::artist::Options {
            id: None,
            search: None,
            song_id: None,
            release_id: None,
            page: None,
            per_page: None,
        };

        if let Some(id) = id {
            options.id = Some(id);
        }

        if let Some(search) = search {
            options.search = Some(search);
        }

        if let Some(song_id) = song_id {
            options.song_id = Some(song_id);
        }

        if let Some(release_id) = release_id {
            options.release_id = Some(release_id);
        }

        Ok(crate::database::artist::get_artist(&options, db).await.unwrap())
    }

    async fn Page<'ctx>(page: Option<i32>, per_page: Option<i32>, _context: &'ctx Context) -> FieldResult<Page> {
        let page_info = PageInfo {
            total: 0,
            per_page,
            current_page: page,
            last_page: 0,
        };

        /*println!("{:?}", page_info);
        let s = executor.location();
        println!("{:?}", s);*/

        // Return an error if there are two different fields in the look_ahead.
        /* let look_ahead = executor.look_ahead();
        let query_typed = look_ahead.children();
        for child in query_typed {
            println!("{:?}", child);
        } */
        //println!("{:?}", page_children);


        Ok(Page {
            page_info,
        })
    }
}

pub struct MutationRoot;

#[graphql_object(context = Context)]
impl MutationRoot {
    async fn create_song(
        context: &Context,
        input: NewSong,
    ) -> FieldResult<Song> {
        // Ok(Song)
        let ulid = ulid::Ulid::new();
        let name = Name {
            native: input.name.native,
            romanized: input.name.romanized,
            english: input.name.english,
        };
        let artists = input.artists;
        let releases = input.releases;

        Ok(crate::database::song::create_song(ulid, name, artists, Some(releases), &*context.db).await.unwrap())
        //todo!()
    }
}

pub fn make_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot, EmptySubscription::<Context>::new())
}

async fn deserialize_body(body: Body) -> Result<GraphQLRequest, io::Error> {
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let request: Result<GraphQLRequest, serde_json::Error> =
        serde_json::from_slice(&bytes.to_vec());

    Ok(request.unwrap())
}
