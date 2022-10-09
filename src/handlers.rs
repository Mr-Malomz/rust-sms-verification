use actix_web::{post, web::Json, HttpResponse};
use reqwest::StatusCode;

use crate::{
    models::{APIResponse, OTPData, VerifyOTPData},
    services::TwilioService,
};

#[post("/otp")]
pub async fn send_otp(new_data: Json<OTPData>) -> HttpResponse {
    let data = OTPData {
        phone_number: new_data.phone_number.clone(),
    };

    let otp_details = TwilioService::send_otp(&data.phone_number).await;

    match otp_details {
        Ok(otp) => HttpResponse::Ok().json(APIResponse {
            status: StatusCode::ACCEPTED.as_u16(),
            message: "success".to_string(),
            data: otp.sid,
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: e.to_string(),
        }),
    }
}

#[post("/verifyOTP")]
pub async fn verify_otp(new_data: Json<VerifyOTPData>) -> HttpResponse {
    let data = VerifyOTPData {
        user: new_data.user.clone(),
        code: new_data.code.clone(),
    };

    let otp_details = TwilioService::verify_otp(&data.user.phone_number, &data.code).await;

    match otp_details {
        Ok(_) => HttpResponse::Ok().json(APIResponse {
            status: StatusCode::ACCEPTED.as_u16(),
            message: "success".to_string(),
            data: "OTP verified successfully".to_string(),
        }),
        Err(e) => HttpResponse::InternalServerError().json(APIResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "failure".to_string(),
            data: e.to_string(),
        }),
    }
}
