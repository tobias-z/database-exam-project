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
use crate::{monitor_service, connection};
use crate::{alert::Alerter, log_service, model::MonitorQuery};

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
        warn!("Unable to create monitor query using request: {:?}", monitor_query.0);
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
    let client = connection::get_client().await.expect("unable to connect to mongodb");
    let db = client.database("logs");
    let mut session = client.start_session(None).await.unwrap();
    // Ensure that if we do find a MonitorQuery it will not be deleted in some other process in the
    // meantime (ACID)
    session.start_transaction(None).await.unwrap();
    let Ok(Some(monitor_query)) = monitor_service::get_monitor_query_by_id(&db, &id).await else {
        info!("No MonitorQuery found with id: {}", &id);
        return Err(WebError::new(Status::NotFound, format!("No monitor query found with id {}", &id)).into());
    };
    if let Err(e) = monitor_service::delete_monitor_query(&db, &id).await {
        error!("Unable to delete MonitorQuery with id: {}. {}", id, e);
        return Err(WebError::new(Status::InternalServerError, e.to_string()).into());
    };
    session.commit_transaction().await.unwrap();

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
