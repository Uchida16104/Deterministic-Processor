use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use serde_json::Value;

use crate::models::request::ProcessRequest;
use crate::models::response::{ProcessResponse, ErrorResponse};
use crate::processing::pipeline::ProcessingPipeline;
use crate::utils::validation::validate_request;

pub async fn process_data(req: web::Json<ProcessRequest>) -> impl Responder {
    info!("Received processing request for transformation: {}", req.transformation_type);
    
    if let Err(validation_error) = validate_request(&req) {
        error!("Request validation failed: {}", validation_error);
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Validation failed".to_string(),
            message: validation_error,
            details: None,
        });
    }
    
    let pipeline = ProcessingPipeline::new();
    
    match pipeline.process(&req.data, &req.transformation_type, req.parameters.as_ref()) {
        Ok(result) => {
            info!("Processing completed successfully, {} rows returned", result.len());
            HttpResponse::Ok().json(ProcessResponse {
                success: true,
                result,
                message: "Processing completed successfully".to_string(),
                metadata: Some(serde_json::json!({
                    "transformation": req.transformation_type,
                    "input_rows": req.data.len(),
                    "output_rows": result.len()
                })),
            })
        }
        Err(e) => {
            error!("Processing error: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Processing failed".to_string(),
                message: e.to_string(),
                details: Some(serde_json::json!({
                    "transformation": req.transformation_type
                })),
            })
        }
    }
}
