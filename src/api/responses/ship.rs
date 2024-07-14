use serde::Deserialize;

#[derive(Deserialize)]
pub struct Ship {
    symbol: String,
    registration: ShipRegistration,
    crew: ShipCrew,
    frame: ShipFrame,
    reactor: ShipReactor,
    engine: ShipEngine,
    modules: Vec<ShipModule>,
    mounts: Vec<ShipMount>,
}

#[derive(Deserialize)]
struct ShipRegistration {
    name: String,
    factionSymbol: String,
    role: String,
}

#[derive(Deserialize)]
struct ShipCrew {
    current: u64,
    capacity: u64,
    required: u64,
    rotation: String,
    morale: u64,
    wages: u64,
}

#[derive(Deserialize)]
struct ShipFrame {
    symbol: String,
    name: String,
    description: String,
    moduleSlots: u64,
    mountingPoints: u64,
    fuelCapacity: u64,
    condition: u64,
    requirements: ShipRequirements,
}

#[derive(Deserialize)]
struct ShipReactor {
    symbol: String,
    name: String,
    description: String,
    powerOutput: u64,
    condition: u64,
    requirements: ShipRequirements,
}

#[derive(Deserialize)]
struct ShipEngine {
    symbol: String,
    name: String,
    description: String,
    speed: u64,
    condition: u64,
    requirements: ShipRequirements,
}

#[derive(Deserialize)]
struct ShipModule {
    symbol: String,
    capacity: Option<u64>,
    range: Option<u64>,
    name: String,
    description: String,
    requirements: ShipRequirements,
}

#[derive(Deserialize)]
struct ShipMount {
    symbol: String,
    name: String,
    description: String,
    strength: Option<u64>,
    deposits: Option<Vec<String>>,
    requirements: ShipRequirements,
}

#[derive(Deserialize)]
struct ShipRequirements {
    power: u64,
    crew: u64,
    slots: u64,
}

