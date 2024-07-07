use serde::Deserialize;
use reqwest::Client;
use reqwest::Error;
use reqwest::Response;

use crate::contract::*;
use crate::ship::*;
use crate::agent::*;

mod contract; 
mod ship;
mod agent;


enum FactionType
{
    Cosmic,
    Galactic
}


#[derive(Deserialize)]
struct RegistrationResponse {
    token: String,
    agent: Agent,
    contract: Contract,
    faction: Faction,
    ship: Ship,
}



async fn register(callsign: String, faction: FactionType) -> Result<RegistrationResponse, reqwest::Error>
{        
    let client = reqwest::Client::new();
    
    let params = [("callsign", callsign), 
                  ("faction", match faction { 
                      FactionType::Cosmic => String::from("Cosmic"), 
                      FactionType::Galactic => String::from("Galactic")
                  })];

    client
        .post("https://api.spacetraders.io/v2/register")
        .form(&params)
        .send()
        .await?
        .json::<RegistrationResponse>()
        .await
}



fn main() {
    // register(String::from("Testing"), FactionType::Cosmic);


}
