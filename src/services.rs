use std::env;

use dotenv::dotenv;
use reqwest::Client;

use crate::models::OTPResponse;

pub struct TwilioService {}

impl TwilioService {
    fn env_loader(key: &str) -> String {
        dotenv().ok();
        match env::var(key) {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        }
    }

    pub async fn send_otp(phone_number: &String) -> Result<OTPResponse, &'static str> {
        let account_sid = TwilioService::env_loader("TWILIO_ACCOUNT_SID");
        let auth_token = TwilioService::env_loader("TWILIO_AUTHTOKEN");
        let service_id = TwilioService::env_loader("TWILIO_SERVICES_ID");

        let url = format!(
            "https://verify.twilio.com/v2/Services/{serv_id}/Verifications",
            serv_id = service_id
        );

        let body = format!("To={phone}&Channel=sms", phone = phone_number);

        let client = Client::new();
        let res = client
            .post(url)
            .basic_auth(account_sid, Some(auth_token))
            .body(body)
            .send()
            .await;
        println!("{:?}", res.as_ref().ok());

        match res {
            Ok(response) => {
                let result = response.json::<OTPResponse>().await;
                match result {
                    Ok(data) => Ok(data),
                    Err(_) => Err("Error sending OTP"),
                }
            }
            Err(e) => {
                println!("{}", e);
                Err("Error sending OTP")
            }
        }
    }

    pub async fn verify_otp(phone_number: &String, code: &String) -> Result<(), &'static str> {
        let account_sid = TwilioService::env_loader("TWILIO_ACCOUNT_SID");
        let auth_token = TwilioService::env_loader("TWILIO_AUTHTOKEN");
        let service_id = TwilioService::env_loader("TWILIO_SERVICES_ID");

        let url = format!(
            "https://verify.twilio.com/v2/Services/{serv_id}/VerificationCheck",
            serv_id = service_id,
        );

        let body = format!("To={phone}&Code={otp}", phone = phone_number, otp = code);

        let client = Client::new();
        let res = client
            .post(url)
            .basic_auth(account_sid, Some(auth_token))
            .body(body)
            .send()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err("Error verifying OTP"),
        }
    }
}
