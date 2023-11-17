use my_rust_server::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_server().await
}
