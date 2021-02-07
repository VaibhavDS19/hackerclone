use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String,
}

#[derive(Debug, Deserialize)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("This is the first link"),
            link: String::from("https://example.com"),
            author: String::from("Bob"),
        },
        Post {
            title: String::from("This is the second link"),
            link: String::from("https://example.com"),
            author: String::from("Alice"),
        },
    ];

    data.insert("title", "HackerClone");
    data.insert("username", "User");
    data.insert("posts", &posts);
    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn login(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");

    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn submission(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a post");

    let rendered = tera.render("submission.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn process_signup(data: web::Form<User>) -> impl Responder {
    println!("New user signed up: {}", data.username);
    HttpResponse::Ok().body("New signup successful")
}

async fn process_login(data: web::Form<LoginUser>) -> impl Responder {
    println!("Logged in: {}", data.username);
    HttpResponse::Ok().body(format!("Login successful"))
}

async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("Post {} submitted successfully", data.title);
    HttpResponse::Ok().body(format!("Posted successfully"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .route("/", web::get().to(index))
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(process_signup))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/submission", web::get().to(submission))
            .route("/submission", web::post().to(process_submission))
    })
    .bind("0.0.0.0")?
    .bind("127.0.0.1:8000")?
    .bind("192.168.43.29:8000")?
    .run()
    .await
}
