use serde::Deserialize;

#[derive(Deserialize)]
pub struct Contract {
    id: String,
    factionSymbol: String,
    #[serde(rename = "type")]
    contract_type: String,
    terms: ContractTerms,
}

#[derive(Deserialize)]
struct ContractTerms {
    deadline: String,
    payment: ContractPayment,
    deliver: Vec<ContractDeliver>,
}

#[derive(Deserialize)]
struct ContractPayment {
    onAccepted: u64,
    onFulfilled: u64,
}

#[derive(Deserialize)]
struct ContractDeliver {
    tradeSymbol: String,
    destinationSymbol: String,
    unitsRequired: u64,
    unitsFulfilled: u64,
}
