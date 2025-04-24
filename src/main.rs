use askama::Template;
use axum::{
    Router,
    extract::{Form, Path, Request},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
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

enum AppError {
    TemplateError(askama::Error),
}

impl From<askama::Error> for AppError {
    fn from(err: askama::Error) -> Self {
        AppError::TemplateError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::TemplateError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template rendering failed: {}", e),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> Result<Html<String>, AppError> {
    Ok(Html(IndexTemplate {}.render()?))
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
    fn new(contact: Contact) -> Self {
        Self {
            first_name: contact.first_name,
            last_name: contact.last_name,
            email: contact.email,
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

async fn show_contact(Path(_id): Path<u32>, request: Request) -> Result<Html<String>, AppError> {
    let html = match request.headers().get("HX-Request") {
        Some(_) => ContactTemplate::default().as_contact().render()?,
        None => ContactTemplate::default().render()?,
    };

    Ok(Html(html))
}

async fn edit_contact(Path(_id): Path<u32>) -> Result<Html<String>, AppError> {
    Ok(Html(ContactEditTemplate::default().render()?))
}

async fn update_contact(
    Path(_id): Path<u32>,
    Form(contact): Form<Contact>,
) -> Result<Html<String>, AppError> {
    Ok(Html(ContactTemplate::new(contact).as_contact().render()?))
}
