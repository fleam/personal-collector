// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use percent_encoding::percent_decode;
use std::convert::Infallible;
use std::net::SocketAddr;
use tauri::async_runtime;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path();

    if path.starts_with("/fs/") {
        // 提取路径并进行 URL 解码
        let encoded_path = &path["/fs/".len()..];
        let decoded_path = percent_decode(encoded_path.as_bytes()).decode_utf8_lossy();

        // 转换为 Windows 路径格式
        let local_path = decoded_path.replace("/", "\\");
        println!("Local Path: {}", local_path);

        let path_buf = std::path::Path::new(&local_path);
        if !path_buf.exists() {
            println!("Error: File does not exist at path: {}", local_path);
            return Ok(Response::builder()
                .status(404)
                .body(Body::from("File not found"))
                .unwrap());
        }

        match std::fs::read(path_buf) {
            Ok(content) => Ok(Response::new(Body::from(content))),
            Err(e) => {
                println!("Error reading file at {}: {}", local_path, e);
                Ok(Response::builder()
                    .status(500)
                    .body(Body::from(format!("Internal error: {}", e)))
                    .unwrap())
            }
        }
    } else {
        Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap())
    }
}

fn start_fs_server() {
    async_runtime::spawn(async move {
        let addr = SocketAddr::from(([127, 0, 0, 1], 5173));
        let make_svc =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

        // 显式构建 Server 实例并运行
        let server = Server::bind(&addr).serve(make_svc);

        println!("FS HTTP Server running on http://localhost:5173");

        // 直接 await 启动服务器
        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }
    });
}

fn main() {
    start_fs_server(); // 启动 fs HTTP 服务
    personal_collector_lib::run()
}
