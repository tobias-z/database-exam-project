use mongodb::bson::Document;
use rocket::http::Status;
use rocket::{
    response::status::Custom,
    routes,
    serde::{json::Json, Deserialize, Serialize},
    State,
};

use crate::alert::AlertMessage;
use crate::{alert::Alerter, model::MonitorQuery};
use crate::{log_service, monitor_service};

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

#[post(
    "/monitor-query",
    format = "application/json",
    data = "<monitor_query>"
)]
async fn create_monitor_query(
    monitor_query: Json<MonitorQuery>,
    alerter: &State<Alerter>,
) -> WebResult<Json<bool>> {
    info!("Creating new montior query {:?}", monitor_query.0);
    let create = monitor_service::create_monitor_query(&monitor_query).await;
    if let Err(e) = create {
        warn!(
            "Unable to create monitor query using request: {:?}",
            monitor_query.0
        );
        return Err(WebError::new(Status::BadRequest, e.to_string()).into());
    }
    match alerter.alert(AlertMessage::Create(monitor_query.0)) {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(WebError::new(
            Status::InternalServerError,
            "unable to start monitoring process".to_string(),
        )
        .into()),
    }
}

#[get("/monitor-query")]
async fn get_all_monitor_queries() -> WebResult<Json<Vec<Document>>> {
    match monitor_service::get_all_monitor_queries().await {
        Ok(all) => Ok(Json(all)),
        Err(e) => Err(WebError::new(Status::InternalServerError, e.to_string()).into()),
    }
}

#[delete("/monitor-query/<id>")]
async fn delete_monitor_query(id: String, alerter: &State<Alerter>) -> WebResult<Json<bool>> {
    let monitor_query = match monitor_service::delete_and_get_monitor_query(&id).await {
        Ok(query) => query,
        Err(e) => return Err(WebError::new(Status::InternalServerError, e.to_string()).into()),
    };
    match alerter.alert(AlertMessage::Drop(monitor_query)) {
        Ok(_) => Ok(Json(true)),
        Err(_) => Err(WebError::new(
            Status::InternalServerError,
            "unable to remove query monitoring process".to_string(),
        )
        .into()),
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

pub async fn start_rest_server(alerter: Alerter) -> Result<(), rocket::Error> {
    rocket::build()
        .attach(middleware::Logging)
        .manage(alerter)
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
