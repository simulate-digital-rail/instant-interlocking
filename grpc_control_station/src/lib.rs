use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Json,
};
use axum_extra::routing::SpaRouter;
use ixl::{
    interlocking_server::InterlockingServer, ElementStateRequest, ElementStateResponse, MpCommand,
    MtpCommand, Nothing, RacCommand, RlrCommand, RrCommand,
};

use serde_json::{Map, Value};

use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::Duration,
};
use tonic::{transport::NamedService, Request, Response, Status};
use tower_http::cors::CorsLayer;
use track_element::{
    driveway::{DrivewayManager, DrivewayState},
    point::PointState,
    signal::{MainSignalState, SignalState},
    vacancy_section::{self, VacancySectionState},
    TrackElement,
};

pub mod ixl {
    tonic::include_proto!("ixl");
}

#[derive(Clone)]
pub struct WsState {
    driveway_manager: Arc<RwLock<DrivewayManager>>,
}

pub struct ControlStation {
    driveway_manager: Arc<RwLock<DrivewayManager>>,

    topology: String,
    placement: String,
}

impl ControlStation {
    pub fn new<S: Into<String>>(
        driveway_manager: DrivewayManager,
        topology: S,
        placement: S,
    ) -> Self {
        let frontend_path = std::path::Path::new("ixl-frontend/build");
        if !frontend_path.exists() {
            println!(
                "It looks like the interlocking executable is not running in its data directory."
            );
            println!("This means that the React frontend will likely not be available.");
        }

        Self {
            driveway_manager: Arc::new(RwLock::new(driveway_manager)),
            topology: topology.into(),
            placement: placement.into(),
        }
    }

    pub async fn listen(&mut self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let ixl_server = InterlockingServer::new(self.driveway_manager.clone());
        let grpc_path = format!(
            "/{}/:cmd",
            <InterlockingServer<Arc<RwLock<DrivewayManager>>> as NamedService>::NAME
        );

        let grpc_service = tonic_web::enable(ixl_server);

        let frontend_router = SpaRouter::new("/", "ixl-frontend/build");

        let topology = serde_json::from_str::<Value>(&self.topology).unwrap();
        let placement = serde_json::from_str::<Value>(&self.placement).unwrap();
        let axum_router = axum::Router::new()
            .route("/ws", get(ws_handler))
            .route("/topology", get(|| async { Json(topology) }))
            .route("/topology/placement", get(|| async { Json(placement) }))
            .route(&grpc_path, axum::routing::any_service(grpc_service))
            .route(
                "/terminate",
                get(|| async {
                    std::process::exit(0);
                }),
            )
            .layer(CorsLayer::permissive())
            .with_state(WsState {
                driveway_manager: self.driveway_manager.clone(),
            })
            .merge(frontend_router);

        axum::Server::bind(&addr)
            .serve(axum_router.into_make_service())
            .await?;
        Ok(())
    }
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<WsState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: WsState) {
    loop {
        let dw_state = state.driveway_manager.read().unwrap().state();

        if socket
            .send(axum::extract::ws::Message::Text(
                driveway_state_to_json(dw_state).to_string(),
            ))
            .await
            .is_err()
        {
            println!("Client disconnected");
            break;
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

#[tonic::async_trait]
impl ixl::interlocking_server::Interlocking for Arc<RwLock<DrivewayManager>> {
    async fn move_point(&self, command: Request<MpCommand>) -> Result<Response<Nothing>, Status> {
        Ok(Response::new(Nothing {}))
    }

    async fn move_trailed_point(
        &self,
        command: Request<MtpCommand>,
    ) -> Result<Response<Nothing>, Status> {
        Ok(Response::new(Nothing {}))
    }

    async fn reset_axle_counter(
        &self,
        command: Request<RacCommand>,
    ) -> Result<Response<Nothing>, Status> {
        Ok(Response::new(Nothing {}))
    }

    async fn request_route(
        &self,
        command: Request<RrCommand>,
    ) -> Result<Response<Nothing>, Status> {
        println!(
            "Got request for driveway {} - {}",
            &command.get_ref().start,
            &command.get_ref().ziel
        );

        match self
            .write()
            .unwrap()
            .set_driveway(&command.get_ref().start, &command.get_ref().ziel)
        {
            Ok(()) => Ok(Response::new(Nothing {})),
            Err(e) => {
                println!("Error setting driveway: {e:?}");
                Err(Status::invalid_argument(format!(
                    "Invalid driveway {} - {}",
                    &command.get_ref().start,
                    &command.get_ref().ziel
                )))
            }
        }
    }

    async fn release_route(
        &self,
        command: Request<RlrCommand>,
    ) -> Result<Response<Nothing>, Status> {
        Ok(Response::new(Nothing {}))
    }

    async fn get_point_state(
        &self,
        command: Request<ElementStateRequest>,
    ) -> Result<Response<ElementStateResponse>, Status> {
        println!("{}", command.get_ref().element);
        Ok(Response::new(ElementStateResponse {
            state: "left".to_string(),
        }))
    }
}

fn point_state_to_string(state: &PointState) -> &'static str {
    match state {
        PointState::Left => "Left",
        PointState::Right => "Right",
    }
}

fn signal_state_to_string(state: &SignalState) -> &'static str {
    match state.main() {
        MainSignalState::Hp0 => "Hp0",
        MainSignalState::Hp0PlusSh1 => "Hp0",
        MainSignalState::Hp0WithDrivingIndicator => "Hp0",
        MainSignalState::Ks1 => "Ks1",
        MainSignalState::Ks1Flashing => "Ks1",
        MainSignalState::Ks1FlashingWithAdditionalLight => "Ks1",
        MainSignalState::Ks2 => "Ks2",
        MainSignalState::Ks2WithAdditionalLight => "Ks2",
        MainSignalState::Sh1 => "Sh1",
        MainSignalState::IdLight => "CommunicationError",
        MainSignalState::Hp0Hv => "Hp0",
        MainSignalState::Hp1 => "Hp1",
        MainSignalState::Hp2 => "Hp2",
        MainSignalState::Vr0 => "Vr0",
        MainSignalState::Vr1 => "Vr1",
        MainSignalState::Vr2 => "Vr2",
        MainSignalState::Off => "CommunicationError",
    }
}

fn vacancy_section_state_to_string(state: &VacancySectionState) -> &'static str {
    match state {
        VacancySectionState::Free => "Unallocated",
        VacancySectionState::Occupied => "Allocated",
    }
}

fn driveway_state_to_json(state: DrivewayState) -> Value {
    let points = state.points();
    let signals = state.signals();
    let vacancy_sections = state.vacancy_sections();

    let mut output = Value::Object(Map::new());
    output["states"] = Value::Object(Map::new());

    for (point, state) in points {
        output.get_mut("states").unwrap()[point.read().unwrap().id()] =
            point_state_to_string(state).into();
    }

    for (signal, state) in signals {
        output.get_mut("states").unwrap()[signal.read().unwrap().name()] =
            signal_state_to_string(state).into();
    }

    for (vacancy_section, state) in vacancy_sections {
        output.get_mut("states").unwrap()[vacancy_section.read().unwrap().id()] =
            vacancy_section_state_to_string(state).into();
    }

    // TODO
    output["pendingCommand"] = Value::Null;
    output["id"] = Value::Null;
    output["validTransitions"] = Value::Null;
    output
}
