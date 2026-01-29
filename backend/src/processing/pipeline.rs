use log::{debug, info};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

use super::csharp_bridge::{CSharpBridge, CSharpBridgeError};

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("Invalid transformation type: {0}")]
    InvalidTransformation(String),
    
    #[error("C# processing error: {0}")]
    CSharpError(#[from] CSharpBridgeError),
    
    #[error("Data validation error: {0}")]
    ValidationError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

pub struct ProcessingPipeline {
    csharp_bridge: CSharpBridge,
}

impl ProcessingPipeline {
    pub fn new() -> Self {
        info!("Creating new processing pipeline");
        ProcessingPipeline {
            csharp_bridge: CSharpBridge::default(),
        }
    }
    
    pub fn process(
        &self,
        data: &[HashMap<String, Value>],
        transformation_type: &str,
        parameters: Option<&HashMap<String, Value>>,
    ) -> Result<Vec<HashMap<String, Value>>, PipelineError> {
        debug!("Pipeline processing {} rows with transformation: {}", data.len(), transformation_type);
        
        self.validate_data(data)?;
        self.validate_transformation_type(transformation_type)?;
        
        let sorted_data = self.ensure_deterministic_order(data);
        
        let result = self.csharp_bridge.process(
            &sorted_data,
            transformation_type,
            parameters,
        )?;
        
        info!("Pipeline processing completed, {} rows produced", result.len());
        Ok(result)
    }
    
    fn validate_data(&self, data: &[HashMap<String, Value>]) -> Result<(), PipelineError> {
        if data.is_empty() {
            return Err(PipelineError::ValidationError(
                "Input data cannot be empty".to_string()
            ));
        }
        
        if data.len() > 100000 {
            return Err(PipelineError::ValidationError(
                "Input data exceeds maximum allowed size of 100000 rows".to_string()
            ));
        }
        
        Ok(())
    }
    
    fn validate_transformation_type(&self, transformation_type: &str) -> Result<(), PipelineError> {
        let valid_transformations = [
            "normalize",
            "aggregate",
            "filter",
            "transform",
            "sort",
            "deduplicate",
        ];
        
        if !valid_transformations.contains(&transformation_type.to_lowercase().as_str()) {
            return Err(PipelineError::InvalidTransformation(
                format!("Unknown transformation type: {}", transformation_type)
            ));
        }
        
        Ok(())
    }
    
    fn ensure_deterministic_order(&self, data: &[HashMap<String, Value>]) -> Vec<HashMap<String, Value>> {
        let mut sorted_data: Vec<_> = data.iter().cloned().collect();
        
        sorted_data.sort_by(|a, b| {
            let a_json = serde_json::to_string(a).unwrap_or_default();
            let b_json = serde_json::to_string(b).unwrap_or_default();
            a_json.cmp(&b_json)
        });
        
        sorted_data
    }
    
    pub fn get_supported_transformations() -> Vec<&'static str> {
        vec![
            "normalize",
            "aggregate",
            "filter",
            "transform",
            "sort",
            "deduplicate",
        ]
    }
    
    pub fn get_transformation_description(transformation_type: &str) -> Option<&'static str> {
        match transformation_type.to_lowercase().as_str() {
            "normalize" => Some("Normalize numeric values in a specified field to the range [0, 1]"),
            "aggregate" => Some("Group data by a field and aggregate values using sum, avg, min, max, count, or median"),
            "filter" => Some("Filter rows based on a condition applied to a field"),
            "transform" => Some("Apply mathematical transformations like square, sqrt, log, exp, abs, negate, or reciprocal"),
            "sort" => Some("Sort data by a specified field in ascending or descending order"),
            "deduplicate" => Some("Remove duplicate rows based on all fields or specified fields"),
            _ => None,
        }
    }
}

impl Default for ProcessingPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pipeline_creation() {
        let pipeline = ProcessingPipeline::new();
        assert!(ProcessingPipeline::get_supported_transformations().len() > 0);
    }
    
    #[test]
    fn test_validate_empty_data() {
        let pipeline = ProcessingPipeline::new();
        let result = pipeline.validate_data(&vec![]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_transformation_type() {
        let pipeline = ProcessingPipeline::new();
        assert!(pipeline.validate_transformation_type("normalize").is_ok());
        assert!(pipeline.validate_transformation_type("invalid").is_err());
    }
    
    #[test]
    fn test_get_transformation_description() {
        let desc = ProcessingPipeline::get_transformation_description("normalize");
        assert!(desc.is_some());
        assert!(desc.unwrap().contains("Normalize"));
    }
}
