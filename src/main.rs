use axum::{
    extract::Form,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    phone: String,
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/style.css", get(css))
        .route("/contact", post(contact));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Website running at http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn home() -> Html<String> {
    let html = fs::read_to_string("static/index.html")
        .expect("index.html nahi mila");

    Html(html)
}

async fn css() -> impl IntoResponse {
    let css = fs::read_to_string("static/style.css")
        .expect("style.css nahi mila");

    (
        [(axum::http::header::CONTENT_TYPE, "text/css")],
        css,
    )
}

async fn contact(Form(data): Form<ContactForm>) -> Html<String> {
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="UTF-8">
            <title>Enquiry Received</title>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    text-align: center;
                    padding: 100px 20px;
                    background: #fff8f8;
                    color: #333;
                }}

                h1 {{
                    color: #7b1e2b;
                }}

                p {{
                    font-size: 18px;
                }}

                a {{
                    display: inline-block;
                    margin-top: 20px;
                    padding: 12px 25px;
                    background: #7b1e2b;
                    color: white;
                    text-decoration: none;
                    border-radius: 5px;
                }}
            </style>
        </head>

        <body>

            <h1>Thank You, {}!</h1>

            <p>Your enquiry has been received successfully.</p>

            <p>We will contact you soon.</p>

            <a href="/">Back to Hotel Website</a>

        </body>
        </html>
        "#,
        data.name
    ))
}