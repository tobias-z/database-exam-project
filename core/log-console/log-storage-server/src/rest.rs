use mongodb::bson::Document;
use rocket::http::Status;
use rocket::{
    request::{self, FromRequest, Request},
    response::status::Custom,
    routes,
    serde::{json::Json, Deserialize, Serialize},
    State,
};

use crate::alert::AlertMessage;
use crate::monitor_service;
use crate::{alert::AlertHandler, log_service, model::MonitorQuery};

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
    info!("search with query: {}", q);
    match log_service::run_query(q).await {
        Ok(parsed_res) => Ok(Json(parsed_res)),
        Err(err) => Err(WebError::new(Status::UnprocessableEntity, err.to_string()).into()),
    }
}

async fn create_monitor_query(
    monitor_query: Json<MonitorQuery>,
    handler: &State<AlertHandler>,
) -> WebResult<Json<bool>> {
    let create = monitor_service::create_monitor_query(&monitor_query).await;
    if let Err(e) = create {
        return Err(WebError::new(Status::BadRequest, e.to_string()).into());
    }
    match handler.notify(AlertMessage::Create(monitor_query.0)) {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(WebError::new(
            Status::InternalServerError,
            "unable to start monitoring process".to_string(),
        )
        .into()),
    }
}

#[get("/monitor-query")]
async fn get_all_monitor_queries() -> WebResult<Json<Vec<MonitorQuery>>> {
    todo!()
}

#[delete("/monitor-query/<id>")]
async fn delete_monitor_query(id: String) -> WebResult<Json<bool>> {
    todo!()
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

pub async fn start_rest_server(alert_handler: AlertHandler) -> Result<(), rocket::Error> {
    rocket::build()
        .attach(middleware::Logging)
        .manage(alert_handler)
        .mount(
            "/",
            routes![
                ping,
                search,
                create_monitor_query,
                get_all_monitor_queries,
                delete_monitor_query
            ],
        )
        .launch()
        .await?;
    Ok(())
}
