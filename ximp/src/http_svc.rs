use std::io::Cursor;

use axum::body::Bytes;
use axum::debug_handler;
use axum::extract::Query;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::{routing::get, Router};
use hex;
use serde::Deserialize;
use std::io::BufWriter;
use textual_geometry::geometry::{Geometry, SpiralGeometry};
use textual_geometry::rendering::Bitmap;

pub async fn http_svc() {
    let app = Router::new().route("/spiral", get(echo_geometry).post(echo_geometry_lg));

    let addr = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("ximp transcribe listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}

fn spiral_encode_str(s: String) -> (StatusCode, HeaderMap, Vec<u8>) {
    let hex_encoded = hex::encode(s);

    let mut geometry = SpiralGeometry::new(256);
    geometry.translate(hex_encoded);
    let mut bitmap = Bitmap::new(256);
    bitmap.from_geometry(&geometry);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "image/png".parse().unwrap());
    // headers.insert("Content-Disposition", "attachment; filename=\"transcribe.png\"".parse().unwrap());

    let mut buf = BufWriter::new(Cursor::new(Vec::new()));
    bitmap.write_png_to(&mut buf);

    let bytes: Vec<u8> = buf.into_inner().unwrap().into_inner();

    (StatusCode::OK, headers, bytes)
}

#[derive(Deserialize)]
struct GeometryRequest {
    input: String,
}

#[debug_handler]
async fn echo_geometry(query: Query<GeometryRequest>) -> impl IntoResponse {
    spiral_encode_str(query.input.clone())
}

async fn echo_geometry_lg(body: Bytes) -> impl IntoResponse {
    let utf8_request: String;

    if let Ok(decoded) = String::from_utf8(body.to_vec()) {
        utf8_request = decoded;
    } else {
        return (StatusCode::BAD_REQUEST, HeaderMap::new(), Vec::<u8>::new());
    }

    spiral_encode_str(utf8_request)
}
