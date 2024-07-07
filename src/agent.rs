use serde::Deserialize;


#[derive(Deserialize)]
pub struct Agent {
    accountId: String, 
    symbol: String,
    headquarters: String,
    credits: u64,
    startingFaction: String,
    shipCount: u64
}

#[derive(Deserialize)]
struct Chart {
    waypointSymbol: String,
    submittedBy: String,
    submittedOn: String
}

#[derive(Deserialize)]
pub struct Faction {
    symbol: String,
    name: String,
    description: String,
    headquarters: String,
    traits: Vec<FactionTrait>,
}

#[derive(Deserialize)]
struct FactionTrait {
    symbol: String,
    name: String,
    description: String,
}

