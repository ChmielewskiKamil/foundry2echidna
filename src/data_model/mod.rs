use serde::{Deserialize, Serialize};

/*//////////////////////////////////////////////////////////////
                        DATA MODEL STRUCTS
////////////////////////////////////////////////////////////// */
#[derive(Deserialize, Debug, PartialEq)]
pub struct Broadcast {
    pub transactions: Vec<Transaction>,
    pub receipts: Vec<Receipt>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(rename(serialize = "event"))]
    pub transaction_type: String,
    pub contract_address: String,
    pub transaction: TransactionDetails,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub gas_used: String,
    #[serde(rename(serialize = "gas_price"))]
    pub effective_gas_price: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct TransactionDetails {
    pub from: String,
    pub to: Option<String>,
    pub value: String,
    pub data: String,
}
