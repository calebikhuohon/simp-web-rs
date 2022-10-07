mod gcd;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use crate::gcd::gcd;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index)).
            route("/gcd", web::post().to(post_gcd))
    });
    println!("serving on http://localhost:3002....");
    server.bind("127.0.0.1:3002").expect("error binding server to address").run().await.expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
           <title>GCD Calculator</title>
           <form action="/gcd" method="post">
           <input type="text" name="n">
           <input type="text" name="m"/>
           <button type="submit">Compute GCD</button>
           </form>
        "#,
    )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest().content_type("text/html").
            body("computing the gcd with zero is wack. fuck you");
    }

    let response = format!("the greatest common divisor of the numbers {} and {} \
        is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok().content_type("text/html").body(response)
}