use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}
/*
create common diviser, gdc(12,8) -> 4
*/
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        // exchange the value of m, n if m < n
        if m < n {
            let t: u64 = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    // tricky ease to get wrong without semicolon
    n
}

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    println!("Server is listening on localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("Error binding server to")
        .run()
        .await
        .expect("Error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit">Compute GCD</button>
        </form>
        "#,
    )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring");
    }
    let response = format!(
        "The greatest common divisor of the number {} and {} is <b>{}</b>",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}
