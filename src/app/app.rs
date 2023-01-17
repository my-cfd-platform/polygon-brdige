use std::{collections::HashMap, sync::Arc};

use rust_extensions::AppStates;
use tokio::sync::Mutex;

use crate::{settings::SettingsModel, setup_and_start_ws, setup_price_tcp_server, TcpConnection, TcpConnectionNew, setup_price_tcp_server_new};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: Arc<SettingsModel>,
    pub connections: Mutex<HashMap<i32, Arc<TcpConnection>>>,
    pub new_connections: Mutex<HashMap<i32, Arc<TcpConnectionNew>>>,
}

impl AppContext {
    pub fn new(settings: Arc<SettingsModel>) -> AppContext {
        AppContext {
            app_states: Arc::new(AppStates::create_initialized()),
            settings,
            connections: Mutex::new(HashMap::new()),
            new_connections: Mutex::new(HashMap::new()),
        }
    }
}

pub async fn setup_and_start(app: &Arc<AppContext>) {
    let app_for_spawn = app.clone();

    setup_and_start_ws(app_for_spawn.clone()).await;
    let tcp_server = setup_price_tcp_server(&app);
    let tcp_server_new = setup_price_tcp_server_new(&app);
    tcp_server.start().await;
    tcp_server_new.start().await;

    app.app_states.set_initialized();
}
