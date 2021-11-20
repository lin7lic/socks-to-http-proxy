use anyhow::Result;
use http::Uri;
use hyper::client::HttpConnector;
use hyper::service::{make_service_fn, service_fn};
use hyper::upgrade::Upgraded;
use hyper::{Body, Request, Response, Server};
use hyper_socks2::SocksConnector;
use std::convert::Infallible;
use std::net::SocketAddr;
use structopt::StructOpt;
use tokio_socks::tcp::Socks5Stream;
use tokio_socks::IntoTargetAddr;

#[derive(StructOpt, Debug)]
#[structopt(name = "sthp", about = "Convert Socks5 proxy into Http proxy")]
struct Cli {
    #[structopt(short, long, default_value = "8080")]
    /// port where Http proxy should listen
    port: u16,

    /// Socks5 proxy address
    #[structopt(short, long, default_value = "127.0.0.1:1080")]
    socks_address: SocketAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::from_args();
    let socks_address = args.socks_address;
    let port = args.port;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let make_service = make_service_fn(move |_| async move {
        Ok::<_, Infallible>(service_fn(move |req| proxy(req, socks_address)))
    });
    let server = Server::bind(&addr)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_service);
    println!("Server is listening on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    };
    Ok(())
}
fn host_addr(uri: &http::Uri) -> Option<String> {
    uri.authority().and_then(|auth| Some(auth.to_string()))
}
async fn proxy(req: Request<Body>, socks_address: SocketAddr) -> Result<Response<Body>> {
    let mut connector = HttpConnector::new();
    connector.enforce_http(false);
    let proxy_addr = socks_address.to_string();
    let proxy_addr = Box::leak(Box::new(format!("socks://{}", proxy_addr.to_string())));
    let proxy_addr = Uri::from_static(proxy_addr);
    if let Some(plain) = host_addr(req.uri()) {
        if req.method() == hyper::Method::CONNECT {
            tokio::task::spawn(async move {
                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, plain, socks_address).await {
                            eprintln!("server io error: {}", e);
                        };
                    }
                    Err(e) => eprintln!("upgrade error: {}", e),
                }
            });
            Ok(Response::new(Body::empty()))
        } else {
            let connector = SocksConnector {
                auth: None,
                proxy_addr,
                connector,
            };
            let client = hyper::Client::builder().build(connector);
            let response = client.request(req).await;
            Ok(response.expect("Cannot make HTTP request"))
        }
    } else {
        let mut resp = Response::new(Body::from("CONNECT must be to a socket address"));
        *resp.status_mut() = http::StatusCode::BAD_REQUEST;
        Ok(resp)
    }
}

async fn tunnel<'t, I>(mut upgraded: Upgraded, plain: I, socks_address: SocketAddr) -> Result<()>
where
    I: IntoTargetAddr<'t>,
{
    let mut stream = Socks5Stream::connect(socks_address, plain).await?;

    // Proxying data
    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut stream).await?;

    // Print message when done
    println!(
        "client wrote {} bytes and received {} bytes",
        from_client, from_server
    );
    Ok(())
}
