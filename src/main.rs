use async_std::task::{sleep, spawn};
use std::time::Duration;

const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 8765;

#[async_std::main]
async fn main() {
    spawn(run_server());
    sleep(Duration::from_secs(1)).await;
    let header_name =
        surf::http_types::headers::HeaderName::from_ascii(b"Cookie".to_vec()).unwrap();
    let req = surf::get(format!("http://{}:{}/", HOST, PORT))
        .set_header(header_name, "a=a-val; b=b-val; c=c-val");
    req.await.unwrap();
}

async fn run_server() {
    let mut app = tide::new();
    app.at("/").get(|req: tide::Request<()>| {
        async move {
            dbg!(req.cookie("a"));
            dbg!(req.cookie("b"));
            dbg!(req.cookie("c"));
            Ok(tide::Response::new(tide::StatusCode::Ok))
        }
    });
    app.listen((HOST, PORT)).await.unwrap();
}
