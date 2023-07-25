use serde::{Deserialize, Serialize};
use serde_json::Value; 

#[derive(Debug, Serialize)]
pub struct FlutterwaveChargeRequest<'a> {
    pub tx_ref: &'a str,
    pub amount: &'a str,
    pub currency: &'a str,
    pub redirect_url: &'a str,
    pub payment_options: Option<Value>, // Use serde_json::Value for handling dynamic JSON fields
    pub meta: Option<Value>,
    pub customer: Option<Value>,
    pub customizations: Option<Value>,
    pub subaccounts: Option<Value>,
    pub payment_plan: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct FlutterwaveChargeResponse {
    pub status: String,
    pub message: String,
    pub data: FlutterwaveChargeData,
}

#[derive(Debug, Deserialize)]
pub struct FlutterwaveChargeData {
    pub link: String,
}

#[derive(Debug, Serialize)]
pub struct FlutterwaveVerifyRequest<'a> {
    pub tx_ref: &'a str,
    pub otp: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct FlutterwaveVerifyResponse {
    pub status: String,
    pub message: String,
    pub data: FlutterwaveVerifyData,
}

#[derive(Debug, Deserialize)]
pub struct FlutterwaveVerifyData {
    pub tx_ref: String,
    pub flw_ref: String,
    pub amount: String,
    pub currency: String,
    pub charged_amount: String,
}

pub async fn create_flutterwave_charge(
    api_key: &str,
    secret_key: &str,
    charge_request: FlutterwaveChargeRequest<'_>,
) -> Result<FlutterwaveChargeResponse, Box<dyn std::error::Error>> {
    let url = "https://api.flutterwave.com/v3/payments";

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", secret_key))
        .header("Content-Type", "application/json")
        .json(&charge_request)
        .send()
        .await?
        .json::<FlutterwaveChargeResponse>()
        .await?;

    Ok(res)
}

pub async fn verify_flutterwave_charge(
    api_key: &str,
    secret_key: &str,
    verify_request: FlutterwaveVerifyRequest<'_>,
) -> Result<FlutterwaveVerifyResponse, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.flutterwave.com/v3/transactions/{}/verify",
        verify_request.tx_ref
    );

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", secret_key))
        .header("Content-Type", "application/json")
        .json(&verify_request)
        .send()
        .await?
        .json::<FlutterwaveVerifyResponse>()
        .await?;

    Ok(res)
    //TODO
    //let charge_response = create_flutterwave_charge(api_key, secret_key, charge_request).await?;
//println!("Charge response: {:?}", charge_response);

// Check if the charge was successful before proceeding with verification
//if charge_response.status == "success" {
    // In a real-world scenario, you would redirect the user to the URL returned in `charge_response.data.link`
    // for them to make the payment, and then use the following code to verify the payment
    // after the user has completed the payment process and returned to your website.
    //let verify_request = FlutterwaveVerifyRequest {
        //tx_ref,
        //otp: "12345",
   // };
    //let verify_response = verify_flutterwave_charge(api_key, secret_key, verify_request).await?;
   // println!("Verify response: {:?}", verify_response);
//} else {
    // Handle the failed charge here (e.g., display an error message to the user)
    //println!("Charge failed: {:?}", charge_response.message);
//}

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "YOUR_API_KEY";
    let secret_key = "YOUR_SECRET_KEY";
    let tx_ref = "YOUR_TRANSACTION_REF";
    let amount = "1000";
    let currency = "KES";
    let redirect_url = "https://example.com/callback";
    let payment_type = "card";

    let charge_request = FlutterwaveChargeRequest {
        
        tx_ref,
        amount,
        currency,
        redirect_url,
        payment_options, // Use serde_json::Value for handling dynamic JSON fields
        meta,
        customer,
        customizations,
        subaccounts,
        payment_plan
    };

    let charge_response = create_flutterwave_charge(api_key, secret_key, charge_request).await?;
    println!("Charge response: {:?}", charge_response);

    // In a real-world scenario, you would redirect the user to the URL returned in `charge_response.data.link`
    // for them to make the payment, and then use the following code to verify the payment
    // after the user has completed the payment process and returned to your website.
    let verify_request = FlutterwaveVerifyRequest {
        tx_ref,
        otp: "12345",
    };
    let verify_response = verify_flutterwave_charge(api_key, secret_key, verify_request).await?;
    println!("Verify response: {:?}", verify_response);

    Ok(())
}
