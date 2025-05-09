use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = "Hello, World!";
    Ok(Response::new(Body::from(response)))
}

#[tokio::main]
async fn main() {
    // 定义服务器地址和端口号
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    // 创建服务处理函数
    let make_svc = make_service_fn(|_conn: &hyper::server::conn::AddrStream| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    // 创建 HTTP 服务器对象
    let server = Server::bind(&addr).serve(make_svc);

    println!("服务器已启动，监听端口 8000...");

    // 启动服务器，一直运行直到手动停止
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}