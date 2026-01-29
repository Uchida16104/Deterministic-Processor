use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProcessRequest {
    #[serde(rename = "data")]
    pub data: Vec<HashMap<String, serde_json::Value>>,
    
    #[serde(rename = "transformationType")]
    pub transformation_type: String,
    
    #[serde(rename = "parameters")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
}

impl ProcessRequest {
    pub fn new(
        data: Vec<HashMap<String, serde_json::Value>>,
        transformation_type: String,
        parameters: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        ProcessRequest {
            data,
            transformation_type,
            parameters,
        }
    }
    
    pub fn is_valid(&self) -> bool {
        !self.data.is_empty() && !self.transformation_type.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_request_creation() {
        let data = vec![HashMap::new()];
        let req = ProcessRequest::new(
            data,
            "normalize".to_string(),
            None,
        );
        assert!(req.is_valid());
    }
    
    #[test]
    fn test_process_request_invalid() {
        let req = ProcessRequest::new(
            vec![],
            "normalize".to_string(),
            None,
        );
        assert!(!req.is_valid());
    }
}
