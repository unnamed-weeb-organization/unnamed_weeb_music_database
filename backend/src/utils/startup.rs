use hyper::{server::conn::AddrIncoming, Body, Server};
use routerify::{Middleware, Router, RouterService};
use std::{convert::Infallible, net::SocketAddr};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
pub type ServerStart = Server<AddrIncoming, RouterService<Body, Infallible>>;

pub fn up(conf: super::config::Config) -> (ServerStart, SocketAddr) {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let router: Router<Body, Infallible> = Router::builder()
        .middleware(Middleware::pre(super::middleware::logger))
        .middleware(Middleware::post(super::middleware::setup_cors))
        .build()
        .unwrap();

    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::new(conf.ip, conf.port);
    let server = Server::bind(&addr).serve(service);

    (server, addr)
}
