use crate::models::Song;
use hyper::{Body, Request, Response};
use juniper::{
    http::{graphiql::graphiql_source, GraphQLRequest},
    EmptyMutation, EmptySubscription, FieldResult, RootNode,
};
use routerify::prelude::*;
use std::{io, sync::Arc};

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;
type LocalSchema = Arc<Schema>;

pub async fn graphiql(_: Request<Body>) -> Result<Response<Body>, io::Error> {
    let html = graphiql_source("/graphql", None);
    Ok(Response::new(Body::from(html)))
}

pub async fn graphql(req: Request<Body>) -> Result<Response<Body>, io::Error> {
    let data = req.data::<LocalSchema>().unwrap().clone();
    let request = deserialize_body(req.into_body()).await?;
    let response = request.execute(&data, &()).await;

    Ok(Response::new(Body::from(
        serde_json::to_string(&response).unwrap(),
    )))
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn song(_id: String) -> FieldResult<Song> {
        Ok(Song::default())
    }
}

pub fn make_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

async fn deserialize_body(body: Body) -> Result<GraphQLRequest, io::Error> {
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    let request: Result<GraphQLRequest, serde_json::Error> =
        serde_json::from_slice(&bytes.to_vec());

    Ok(request.unwrap())
}
