
use libsample::common;
use libsample::util;
use serde::{Deserialize, Serialize};
use actix_web::middleware::Logger;
use actix_web::{
    web, App,
    HttpRequest, HttpResponse, HttpServer,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HTTPStatusMsg {
    pub status: String,
    pub status_message: String,
}

impl HTTPStatusMsg {
    pub fn success(msg: &str) -> HTTPStatusMsg {
        HTTPStatusMsg {
            status: String::from("success"),
            status_message: String::from(msg),
        }
    }
}

async fn example_route(_req: HttpRequest) -> HttpResponse {
    let result = util::math::add(2, 3);
    let msg = format!("{}", result);
    HttpResponse::Ok().json(HTTPStatusMsg::success(&msg))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    common();
    println!("Launch Server on 0.0.0.0:3030 🚀");

    HttpServer::new(move || {
        App::new()
            .route(
                "/test",
                web::get().to(example_route),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("0.0.0.0", 3030))?
    .run()
    .await
}
