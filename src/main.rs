use axum::{
    routing::{get, put, post},
    http::StatusCode,
    Json, Router, extract::Path, response::Html,
};
use askama::Template;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with routes
    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/contact/{id}", get(show_contact))
        .route("/contact/{id}/edit", get(edit_contact))
        .route("/contact/{id}", put(update_contact));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

// Template for displaying contact details
#[derive(Template)]
#[template(path = "contact.html")]
struct ContactTemplate {
    first_name: String,
    last_name: String,
    email: String,
}

// Template for editing contact details
#[derive(Template)]
#[template(path = "edit_contact.html")]
struct EditContactTemplate {
    first_name: String,
    last_name: String,
    email: String,
}

// Handler to show contact details
async fn show_contact(Path(id): Path<u32>) -> Html<String> {
    let template = ContactTemplate {
        first_name: "Joe".to_string(),
        last_name: "Blow".to_string(),
        email: "joe@blow.com".to_string(),
    };
    Html(template.render().unwrap())
}

// Handler to show the edit form
async fn edit_contact(Path(id): Path<u32>) -> Html<String> {
    let template = EditContactTemplate {
        first_name: "Joe".to_string(),
        last_name: "Blow".to_string(),
        email: "joe@blow.com".to_string(),
    };
    Html(template.render().unwrap())
}

// Handler to update contact details
async fn update_contact(Path(id): Path<u32>) -> Html<&'static str> {
    Html("<div>Contact updated successfully!</div>")
}