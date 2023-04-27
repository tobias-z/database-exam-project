use mongodb::bson::Document;
use rocket::{
    http::Status,
    response::status::Custom,
    routes,
    serde::{json::Json, Deserialize, Serialize},
};

use crate::query_lang;

type ErrorResponse = Custom<Json<WebError>>;

type WebResult<T> = Result<T, ErrorResponse>;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct WebError {
    code: u16,
    message: String,
}

impl WebError {
    fn new(status: Status, message: String) -> Self {
        Self {
            code: status.code,
            message,
        }
    }
}

impl From<WebError> for ErrorResponse {
    fn from(error: WebError) -> Self {
        Custom(Status { code: error.code }, Json(error))
    }
}

#[get("/ping")]
async fn ping() -> String {
    "pong".to_string()
}

#[get("/search?<q>")]
async fn search(q: &str) -> WebResult<Json<Vec<Document>>> {
    match query_lang::parse::into_aggregation(q) {
        Ok(parsed_res) => Ok(Json(parsed_res)),
        Err(err) => Err(WebError::new(Status::UnprocessableEntity, err.to_string()).into()),
    }
}

mod middleware {
    use rocket::{
        fairing::{Fairing, Info, Kind},
        Data, Request,
    };

    pub struct Logging;

    #[rocket::async_trait]
    impl Fairing for Logging {
        fn info(&self) -> Info {
            Info {
                name: "Logging",
                kind: Kind::Request,
            }
        }

        // ensure that we log all requests to our system
        async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
            match request.real_ip() {
                Some(ip) => info!("{}: {} {}", ip, request.method(), request.uri()),
                None => info!("{} {}", request.method(), request.uri()),
            }
        }
    }
}

pub async fn run_rest_server() -> Result<(), rocket::Error> {
    rocket::build()
        .attach(middleware::Logging)
        .mount("/", routes![ping, search])
        .launch()
        .await?;
    Ok(())
}
