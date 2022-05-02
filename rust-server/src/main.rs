
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

// mod mandelbrot;

fn main() {
    println!("Hello, world!");
    println!("This is an excellent decision");
    let server = HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(get_index))
        .route("/mandelbrot", web::get().to(get_brat_form))
        // .route("/mandelbrot", web::post().to(post_brat))
    });
    
    println!("Serving on http://localhost:3000...");
    
    server.bind("0.0.0.0:3000").expect("error binding server to address")
        .run().expect("error running server");

}

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>This is my response</title>
            <p>This is some text</p>
            <a href="./mandelbrot">Mandelbrot</a>
            "#,
            )
}

fn get_brat_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <form action="/brat" method="post">
            <input type="text" name="bounds">
            <input type="text" name="upper_left">
            <input type="text" name="lower_right">
            <button type="submit"> Generate brat</button>
            </form>
            "#,)
}

fn post_brat(form: web::Form<BratParams>) -> HttpResponse {
    if form.bounds.0 < 1 || form.bounds.1 < 1 || form.upper_left.len() != 2 || form.lower_right.len() != 2 {
        return HttpResponse::BadRequest()
                    .content_type("text/html")
                    .body("Some part of that didn't work.");
    }

    return HttpResponse::BadRequest()
        .content_type("text/html")
        .body("not implemented yet.");

}




#[derive(Deserialize)]
struct BratParams { // TODO: this needs to be able to read strings, which can be parsed appropriately inside mandelbrot
    bounds: (u64, u64),
    upper_left: Vec<u8>,
    lower_right: Vec<u8>,
}

#[test]
fn test_test() {
    print!("You're beautiful baby");
    assert_eq!(true, true);
}
