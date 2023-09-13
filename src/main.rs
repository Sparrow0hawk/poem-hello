use poem::{
    get, handler, listener::TcpListener, middleware::Tracing, web::Path, EndpointExt,
    Route, Server
};
use tokio;

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("Hello! {name}")
}

pub fn create_app() -> Route {
    let app = Route::new().at("/hello/:name", get(hello).with(Tracing));

    app
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG","poem=debug");
    }

    let app = create_app();
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("hello-world")
        .run(app)
        .await
}

#[cfg(test)]
mod tests {
    use poem::{test::TestClient};
    use super::*;
    #[tokio::test]
    async fn test_hello() {

        let app = create_app();

        let cli = TestClient::new(app);

        let resp = cli.get("/hello/foo").send().await;

        resp.assert_status_is_ok();

        resp.assert_text("Hello! foo").await;

    }



}