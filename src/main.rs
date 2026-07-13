use actix_files::{Files, NamedFile};
use actix_web::{
    http::header,
    web, App, HttpResponse, HttpServer, Responder,
};
use html_escape::encode_text;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rusqlite::{params, Connection};
use std::sync::Mutex;

struct AppState {
    db: Mutex<Connection>,
}

#[derive(serde::Deserialize)]
struct FormData {
    content: String,
}

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("index.html"))
}

async fn submit(
    form: web::Form<FormData>,
    data: web::Data<AppState>,
) -> impl Responder {
    let token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let conn = data.db.lock().unwrap();

    conn.execute(
        "INSERT INTO pastes (token, content) VALUES (?, ?)",
        params![token, form.content],
    )
    .expect("Failed to insert paste");

    HttpResponse::SeeOther()
        .append_header((header::LOCATION, format!("/paste/{}", token)))
        .finish()
}

async fn get_paste(
    token: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let conn = data.db.lock().unwrap();

    let result: Result<String, _> = conn.query_row(
        "SELECT content FROM pastes WHERE token=?",
        params![token.to_string()],
        |row| row.get(0),
    );

    match result {
        Ok(content) => HttpResponse::Ok()
            .content_type("text/html")
            .body(format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Rust PasteBin</title>
<link rel="stylesheet" href="/style.css">
</head>

<body>

<h1>Rust PasteBin</h1>

<pre>{}</pre>

</body>
</html>"#,
                encode_text(&content)
            )),

        Err(_) => HttpResponse::NotFound()
            .body("Paste not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Connection::open("pastes.db")
        .expect("Unable to open database");

    db.execute(
        "CREATE TABLE IF NOT EXISTS pastes(
            token TEXT PRIMARY KEY,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .expect("Failed to create table");

    let app_state = web::Data::new(AppState {
        db: Mutex::new(db),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())

            // Serve CSS
            .route(
                "/style.css",
                web::get().to(|| async {
                    NamedFile::open("src/style.css")
                }),
            )

            // Optional: serve all files in src
            .service(Files::new("/static", "./src"))

            .route("/", web::get().to(index))
            .route("/submit", web::post().to(submit))
            .route("/paste/{token}", web::get().to(get_paste))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}