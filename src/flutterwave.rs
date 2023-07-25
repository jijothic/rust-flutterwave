use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct FlutterwaveChargeRequest<'a> {
    tx_ref: &'a str,
    amount: &'a str,
    currency: &'a str,
    redirect_url: &'a str,
    payment_type: &'a str,
}

#[derive(Debug, Deserialize)]
struct FlutterwaveChargeResponse {
    status: String,
    message: String,
    data: FlutterwaveChargeData,
}

#[derive(Debug, Deserialize)]
struct FlutterwaveChargeData {
    link: String,
}

#[derive(Debug, Serialize)]
struct FlutterwaveVerifyRequest<'a> {
    tx_ref: &'a str,
    otp: &'a str,
}

#[derive(Debug, Deserialize)]
struct FlutterwaveVerifyResponse {
    status: String,
    message: String,
    data: FlutterwaveVerifyData,
}

#[derive(Debug, Deserialize)]
struct FlutterwaveVerifyData {
    tx_ref: String,
    flw_ref: String,
    amount: String,
    currency: String,
    charged_amount: String,
}

async fn  create_flutterwave_charge(
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

async fn verify_flutterwave_charge(
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
        payment_type,
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
