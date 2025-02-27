use crate::{
    api::{Api, Authenticated},
    repl::commands::Command,
};

pub struct AppState {
    pub api: Api<Authenticated>,
    pub commands: Vec<Command>,
}
