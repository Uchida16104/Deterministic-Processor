use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessResponse {
    pub success: bool,
    pub result: Vec<HashMap<String, serde_json::Value>>,
    pub message: String,
    pub metadata: Option<serde_json::Value>,
}

impl ProcessResponse {
    pub fn success(
        result: Vec<HashMap<String, serde_json::Value>>,
        message: String,
    ) -> Self {
        ProcessResponse {
            success: true,
            result,
            message,
            metadata: None,
        }
    }
    
    pub fn with_metadata(
        result: Vec<HashMap<String, serde_json::Value>>,
        message: String,
        metadata: serde_json::Value,
    ) -> Self {
        ProcessResponse {
            success: true,
            result,
            message,
            metadata: Some(metadata),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(error: String, message: String) -> Self {
        ErrorResponse {
            error,
            message,
            details: None,
        }
    }
    
    pub fn with_details(
        error: String,
        message: String,
        details: serde_json::Value,
    ) -> Self {
        ErrorResponse {
            error,
            message,
            details: Some(details),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_response_creation() {
        let response = ProcessResponse::success(
            vec![],
            "Success".to_string(),
        );
        assert!(response.success);
        assert_eq!(response.message, "Success");
    }
    
    #[test]
    fn test_error_response_creation() {
        let error = ErrorResponse::new(
            "TestError".to_string(),
            "Test message".to_string(),
        );
        assert_eq!(error.error, "TestError");
        assert_eq!(error.message, "Test message");
    }
}
