mod flutterwave;

use flutterwave::{
    create_flutterwave_charge, FlutterwaveChargeRequest, FlutterwaveVerifyRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "YOUR_API_KEY";
    let secret_key = "YOUR_SECRET_KEY";
    let tx_ref = "YOUR_TRANSACTION_REF";
    let amount = "10";
    let currency = "KES";
    let redirect_url = "http://example.com";
    let charge_request = FlutterwaveChargeRequest {
        tx_ref,
        amount,
        currency,
        redirect_url,
        payment_options: None,
        meta: None,
        customer: None,
        customizations: None,
        subaccounts: None,
        payment_plan: None,
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
