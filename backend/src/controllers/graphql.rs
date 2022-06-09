use crate::{models::song::Song, utils::context::Context};
use hyper::{Body, Request, Response};
use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptyMutation, EmptySubscription, FieldResult, RootNode, graphql_object,
};
use routerify::prelude::*;
use std::{io, sync::Arc};

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;
type LocalSchema = Arc<Schema>;

pub async fn graphiql(_: Request<Body>) -> Result<Response<Body>, io::Error> {
    let html = graphiql_source("/graphql", None);
    Ok(Response::new(Body::from(html)))
}

pub async fn graphql(req: Request<Body>) -> Result<Response<Body>, io::Error> {
    let data = req.data::<LocalSchema>().unwrap().clone();
	let ctx = req.data::<Context>().unwrap().clone();
    let request = deserialize_body(req.into_body()).await?;
    let response = request.execute(&data, &ctx).await;

    Ok(Response::new(Body::from(
        serde_json::to_string(&response).unwrap(),
    )))
}

pub struct QueryRoot;

#[graphql_object(context = Context)]
impl QueryRoot {
    async fn song<'ctx>(id: String, context: &'ctx Context) -> FieldResult<Song> {
        // Ok(Song)
		let s = Song::get_song_by_id(id, &*context.db).await.unwrap();
		Ok(s)
        //todo!()
    }
}

pub fn make_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::<Context>::new(), EmptySubscription::<Context>::new())
}

async fn deserialize_body(body: Body) -> Result<GraphQLRequest, io::Error> {
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let request: Result<GraphQLRequest, serde_json::Error> =
        serde_json::from_slice(&bytes.to_vec());

    Ok(request.unwrap())
}
