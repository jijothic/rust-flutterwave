use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct FlutterwaveChargeRequest<'a> {
    pub tx_ref: &'a str,
    pub amount: &'a str,
    pub currency: &'a str,
    pub redirect_url: &'a str,
    pub payment_options: Option<&'a str>,
    pub meta: Option<&'a str>,
    pub customer: Option<FlutterwaveCustomer<'a>>,
    pub customizations: Option<FlutterwaveCustomizations<'a>>,
    pub subaccounts: Option<Vec<FlutterwaveSubaccount<'a>>>,
    pub payment_plan: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct FlutterwaveChargeResponse {
    pub status: &'static str,
    pub message: &'static str,
    pub data: FlutterwaveChargeData,
}

#[derive(Debug, Deserialize)]
struct FlutterwaveChargeData {
    pub link: String,
}

#[derive(Debug, Serialize)]
struct FlutterwaveCustomer<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub phone_number: &'a str,
}

#[derive(Debug, Serialize)]
struct FlutterwaveCustomizations<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub logo: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct FlutterwaveSubaccount<'a> {
    pub id: &'a str,
    pub transaction_charge_type: Option<&'a str>,
    pub transaction_charge: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct FlutterwaveVerifyRequest<'a> {
    pub tx_ref: &'a str,
    pub otp: &'a str,
}

#[derive(Debug, Deserialize)]
struct FlutterwaveVerifyResponse {
    pub status: &'static str,
    pub message: &'static str,
    pub data: Option<FlutterwaveVerifyData>,
}

#[derive(Debug, Deserialize)]
struct FlutterwaveVerifyData {
    pub tx_ref: String,
    pub flw_ref: String,
    pub amount: String,
    pub currency: String,
    pub charged_amount: String,
}
