use hyper::{server::conn::AddrIncoming, Body, Server};
use routerify::{Middleware, Router, RouterService};
use std::{io, net::SocketAddr, sync::Arc};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub type ServerStart = Server<AddrIncoming, RouterService<Body, io::Error>>;

pub fn up(conf: super::config::Config) -> (ServerStart, SocketAddr) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let schema = Arc::new(crate::controllers::graphql::make_schema());

    let router: Router<Body, io::Error> = Router::builder()
        .data(schema)
        .middleware(Middleware::pre(super::middleware::logger))
        .middleware(Middleware::post(super::middleware::setup_cors))
        .scope("/", crate::controllers::handle_routes())
        .err_handler(super::middleware::handle_error)
        .build()
        .unwrap();

    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::new(conf.ip, conf.port);
    let server = Server::bind(&addr).serve(service);

    (server, addr)
}
