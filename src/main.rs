use actix_web::{get, web, App, HttpServer};

// This struct represents state
struct AppState {
    app_name: String,
    app_ver: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    let app_ver = &data.app_ver; // <- get app_ver
    format!("Hello {app_name}!\nVersion: {app_ver}!") // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Hipster Record Store Clerk"),
                app_ver: String::from("0.1"),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}