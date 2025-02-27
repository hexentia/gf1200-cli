use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateLoginRequest<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
