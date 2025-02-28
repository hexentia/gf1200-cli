use std::{env, fmt::Display};

use inquire::Text;
use reqwest::blocking::{Client, RequestBuilder, Response};
use secrecy::{ExposeSecret, SecretString};
use serde::de::DeserializeOwned;
use serde_json::json;
use types::{
    requests::UpdateLoginRequest, responses::SessionResponse, ConnectedDevice, Device, LanStatus,
    WanStatus,
};

use crate::utils::ui::SafePrompt;

mod routes;
pub mod types;

pub trait ApiState {}
pub struct Unauthenticated;
pub struct Authenticated {
    token: SecretString,
    username: SecretString,
    password: SecretString,
}
impl ApiState for Unauthenticated {}
impl ApiState for Authenticated {}
pub struct Api<State: ApiState> {
    url: String,
    pub client: Client,
    state: State,
}
impl<State: ApiState> Api<State> {
    pub fn with_route<T: Display>(&self, route: T) -> String {
        format!("{}{}", self.url, route)
    }
}
impl Api<Unauthenticated> {
    fn prompt_router_addr() -> String {
        env::var("GF1200_ADDR").ok().unwrap_or_else(|| {
            let addr = Text::new("<endereço>")
                .with_help_message("Deixe em branco pra usar '192.168.0.1'")
                .safely_prompt();
            if addr.is_empty() {
                "192.168.0.1".to_string()
            } else {
                addr
            }
        })
    }
    pub fn new() -> Self {
        let router_addr = Self::prompt_router_addr();
        Self {
            url: format!("http://{router_addr}/{}", routes::ROOT),
            client: Client::new(),
            state: Unauthenticated,
        }
    }
    pub fn authenticate(self, username: &str, password: &str) -> Api<Authenticated> {
        let token = self.get_token(username, password);
        let username = username.into();
        let password = password.into();
        Api {
            url: self.url,
            client: self.client,
            state: Authenticated {
                token,
                username,
                password,
            },
        }
    }
    fn get_token(&self, username: &str, password: &str) -> SecretString {
        self.client
            .post(self.with_route(routes::SESSION))
            .json(&json!({
                "username": username,
                "password": password
            }))
            .send()
            .inspect_err(|e| eprintln!("can't send {}: {e}", routes::SESSION))
            .unwrap()
            .error_for_status()
            .map_or_else(
                |_| {
                    eprintln!("login failed. (are the credentials correct?)");
                    std::process::exit(0);
                },
                |response| {
                    response
                        .json::<SessionResponse>()
                        .inspect_err(|e| eprintln!("can't parse {} response: {e}", routes::SESSION))
                        .unwrap()
                        .token
                        .into()
                },
            )
    }
}
impl Api<Authenticated> {
    pub fn token(&self) -> String {
        self.state.token.expose_secret().to_string()
    }
    pub fn get<'b>(&self, route: &'b str) -> RequestBuilder {
        self.client
            .get(self.with_route(route))
            .header("authorization", format!("Bearer {}", self.token()))
    }
    pub fn post<'b>(&self, route: &'b str) -> RequestBuilder {
        self.client.post(self.with_route(route))
    }
    pub fn put<'b>(&self, route: &'b str) -> RequestBuilder {
        self.client
            .put(self.with_route(route))
            .header("authorization", format!("Bearer {}", self.token()))
    }
    pub fn connected_devices(&self) -> Option<Vec<ConnectedDevice>> {
        self.get(routes::DEVICES).send().map_or_else(
            |e| {
                eprintln!("failed to GET {}: {e:?}", routes::DEVICES);
                None
            },
            parse_json_response,
        )
    }
    pub fn connected_device(&self, mac_address: &str) -> Option<ConnectedDevice> {
        self.get(&format!("/connected_device/{mac_address}"))
            .send()
            .map_or_else(
                |e| {
                    eprintln!("failed to GET /connected_device: {e:?}");
                    None
                },
                parse_json_response,
            )
    }
    pub fn restart(&self) -> Result<(), ()> {
        self.put(routes::RESTART)
            .send()
            .map_or_else(|_| Err(()), |_| Ok(()))
    }

    pub fn set_admin_login(
        &self,
        username: Option<&str>,
        password: Option<&str>,
    ) -> Result<(), ()> {
        if username.is_none() && password.is_none() {
            eprintln!(":raised_eyebrow:");
            return Ok(());
        }
        let username = username.unwrap_or(self.state.username.expose_secret());
        let password = password.unwrap_or(self.state.password.expose_secret());

        self.put(routes::SET_LOGIN_CREDS)
            .json(&UpdateLoginRequest {
                id: "0",
                password,
                username,
            })
            .send()
            .map_or_else(|_| Err(()), |_| Ok(()))
    }
    pub fn set_remote_access(&self, _on: bool) -> Result<(), ()> {
        todo!()
    }
    pub fn set_remote_ping(&self, _on: bool) -> Result<(), ()> {
        todo!()
    }
    pub fn wan_status() -> Option<WanStatus> {
        todo!()
    }
    pub fn lan_status(&self) -> Option<LanStatus> {
        self.get(routes::LAN_STATUS)
            .send()
            .map_or_else(|_| None, parse_json_response)
    }
    pub fn device() -> Option<Device> {
        todo!()
    }
}

fn parse_json_response<T: DeserializeOwned>(response: Response) -> Option<T> {
    response
        .json::<T>()
        .inspect_err(|_| eprintln!("failed to parse response."))
        .ok()
}
