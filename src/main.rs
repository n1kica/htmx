use askama::Template;
use axum::{
    Router,
    extract::{Form, Path, Request},
    response::Html,
    routing::{get, put},
};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let styles = ServeDir::new("styles");

    let app = Router::new()
        .route("/", get(index))
        .route("/contact/{id}", get(show_contact))
        .route("/contact/{id}", put(update_contact))
        .route("/contact/{id}/edit", get(edit_contact))
        .nest_service("/styles", styles);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> Html<String> {
    Html(IndexTemplate {}.render().unwrap())
}

#[derive(Deserialize)]
struct Contact {
    first_name: String,
    last_name: String,
    email: String,
}

#[derive(Template)]
#[template(path = "contact.html", blocks = ["contact"])]
struct ContactTemplate {
    first_name: String,
    last_name: String,
    email: String,
}

impl ContactTemplate {
    fn new(first_name: String, last_name: String, email: String) -> Self {
        Self {
            first_name,
            last_name,
            email,
        }
    }
}

impl Default for ContactTemplate {
    fn default() -> Self {
        Self {
            first_name: "Joe".to_string(),
            last_name: "Blow".to_string(),
            email: "joe@blow.com".to_string(),
        }
    }
}

#[derive(Template)]
#[template(path = "contact_edit.html")]
struct ContactEditTemplate {
    first_name: String,
    last_name: String,
    email: String,
}

impl Default for ContactEditTemplate {
    fn default() -> Self {
        Self {
            first_name: "Joe".to_string(),
            last_name: "Blow".to_string(),
            email: "joe@blow.com".to_string(),
        }
    }
}

async fn show_contact(Path(_id): Path<u32>, request: Request) -> Html<String> {
    let html = match request.headers().get("HX-Request") {
        Some(_) => ContactTemplate::default().as_contact().render().unwrap(),
        None => ContactTemplate::default().render().unwrap(),
    };

    Html(html)
}

async fn edit_contact(Path(_id): Path<u32>) -> Html<String> {
    Html(ContactEditTemplate::default().render().unwrap())
}

async fn update_contact(Path(_id): Path<u32>, Form(contact): Form<Contact>) -> Html<String> {
    Html(
        ContactTemplate::new(contact.first_name, contact.last_name, contact.email)
            .as_contact()
            .render()
            .unwrap(),
    )
}
