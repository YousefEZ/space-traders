use reqwest::Client;
use reqwest::Error;
use reqwest::Response;
use serde::Deserialize;

use crate::api::responses::agent::*;
use crate::api::responses::contract::*;
use crate::api::responses::ship::*;

pub enum FactionType {
    Cosmic,
    Galactic,
}

#[derive(Deserialize)]
pub struct RegistrationResponse {
    token: String,
    agent: Agent,
    contract: Contract,
    faction: Faction,
    ship: Ship,
}

pub async fn register(
    callsign: String,
    faction: FactionType,
) -> Result<RegistrationResponse, reqwest::Error> {
    let client = reqwest::Client::new();

    let params = [
        ("callsign", callsign),
        (
            "faction",
            match faction {
                FactionType::Cosmic => String::from("Cosmic"),
                FactionType::Galactic => String::from("Galactic"),
            },
        ),
    ];

    client
        .post("https://api.spacetraders.io/v2/register")
        .form(&params)
        .send()
        .await?
        .json::<RegistrationResponse>()
        .await
}
