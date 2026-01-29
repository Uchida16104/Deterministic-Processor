use crate::models::request::ProcessRequest;
use log::debug;
use regex::Regex;

pub fn validate_request(req: &ProcessRequest) -> Result<(), String> {
    debug!("Validating request with {} data rows", req.data.len());
    
    if !req.is_valid() {
        return Err("Request must contain non-empty data and transformation type".to_string());
    }
    
    validate_data_size(&req.data)?;
    validate_transformation_type(&req.transformation_type)?;
    validate_parameters(&req.transformation_type, req.parameters.as_ref())?;
    
    debug!("Request validation successful");
    Ok(())
}

fn validate_data_size(data: &[std::collections::HashMap<String, serde_json::Value>]) -> Result<(), String> {
    if data.is_empty() {
        return Err("Data array cannot be empty".to_string());
    }
    
    if data.len() > 100000 {
        return Err(format!(
            "Data array contains {} rows, which exceeds the maximum of 100000",
            data.len()
        ));
    }
    
    let max_row_size = 1024 * 10;
    for (index, row) in data.iter().enumerate() {
        let row_json = serde_json::to_string(row).unwrap_or_default();
        if row_json.len() > max_row_size {
            return Err(format!(
                "Row {} exceeds maximum size of {} bytes",
                index, max_row_size
            ));
        }
    }
    
    Ok(())
}

fn validate_transformation_type(transformation_type: &str) -> Result<(), String> {
    let valid_types = [
        "normalize",
        "aggregate",
        "filter",
        "transform",
        "sort",
        "deduplicate",
    ];
    
    let lowercase_type = transformation_type.to_lowercase();
    if !valid_types.contains(&lowercase_type.as_str()) {
        return Err(format!(
            "Invalid transformation type '{}'. Valid types are: {}",
            transformation_type,
            valid_types.join(", ")
        ));
    }
    
    Ok(())
}

fn validate_parameters(
    transformation_type: &str,
    parameters: Option<&std::collections::HashMap<String, serde_json::Value>>,
) -> Result<(), String> {
    let params = match parameters {
        Some(p) => p,
        None => return Ok(()),
    };
    
    match transformation_type.to_lowercase().as_str() {
        "normalize" => validate_normalize_params(params),
        "aggregate" => validate_aggregate_params(params),
        "filter" => validate_filter_params(params),
        "transform" => validate_transform_params(params),
        "sort" => validate_sort_params(params),
        "deduplicate" => validate_deduplicate_params(params),
        _ => Ok(()),
    }
}

fn validate_normalize_params(params: &std::collections::HashMap<String, serde_json::Value>) -> Result<(), String> {
    if !params.contains_key("field") {
        return Err("Normalize transformation requires 'field' parameter".to_string());
    }
    
    if !params["field"].is_string() {
        return Err("'field' parameter must be a string".to_string());
    }
    
    Ok(())
}

fn validate_aggregate_params(params: &std::collections::HashMap<String, serde_json::Value>) -> Result<(), String> {
    if !params.contains_key("groupBy") {
        return Err("Aggregate transformation requires 'groupBy' parameter".to_string());
    }
    
    if !params.contains_key("field") {
        return Err("Aggregate transformation requires 'field' parameter".to_string());
    }
    
    if let Some(operation) = params.get("operation") {
        if !operation.is_string() {
            return Err("'operation' parameter must be a string".to_string());
        }
        
        let valid_operations = ["sum", "avg", "min", "max", "count", "median"];
        let op_str = operation.as_str().unwrap_or("");
        if !valid_operations.contains(&op_str.to_lowercase().as_str()) {
            return Err(format!(
                "Invalid operation '{}'. Valid operations are: {}",
                op_str,
                valid_operations.join(", ")
            ));
        }
    }
    
    Ok(())
}

fn validate_filter_params(params: &std::collections::HashMap<String, serde_json::Value>) -> Result<(), String> {
    if !params.contains_key("field") {
        return Err("Filter transformation requires 'field' parameter".to_string());
    }
    
    if !params.contains_key("condition") {
        return Err("Filter transformation requires 'condition' parameter".to_string());
    }
    
    if !params.contains_key("value") {
        return Err("Filter transformation requires 'value' parameter".to_string());
    }
    
    let valid_conditions = [
        "equals", "notequals", "greaterthan", "lessthan",
        "greaterorequal", "lessorequal", "contains",
        "startswith", "endswith", "matches"
    ];
    
    let condition = params["condition"].as_str().unwrap_or("");
    if !valid_conditions.contains(&condition.to_lowercase().as_str()) {
        return Err(format!(
            "Invalid condition '{}'. Valid conditions are: {}",
            condition,
            valid_conditions.join(", ")
        ));
    }
    
    Ok(())
}

fn validate_transform_params(params: &std::collections::HashMap<String, serde_json::Value>) -> Result<(), String> {
    if !params.contains_key("field") {
        return Err("Transform transformation requires 'field' parameter".to_string());
    }
    
    if let Some(operation) = params.get("operation") {
        if !operation.is_string() {
            return Err("'operation' parameter must be a string".to_string());
        }
        
        let valid_operations = ["square", "sqrt", "log", "exp", "abs", "negate", "reciprocal", "identity"];
        let op_str = operation.as_str().unwrap_or("");
        if !valid_operations.contains(&op_str.to_lowercase().as_str()) {
            return Err(format!(
                "Invalid operation '{}'. Valid operations are: {}",
                op_str,
                valid_operations.join(", ")
            ));
        }
    }
    
    Ok(())
}

fn validate_sort_params(params: &std::collections::HashMap<String, serde_json::Value>) -> Result<(), String> {
    if !params.contains_key("field") {
        return Err("Sort transformation requires 'field' parameter".to_string());
    }
    
    if let Some(direction) = params.get("direction") {
        if !direction.is_string() {
            return Err("'direction' parameter must be a string".to_string());
        }
        
        let valid_directions = ["asc", "desc"];
        let dir_str = direction.as_str().unwrap_or("");
        if !valid_directions.contains(&dir_str.to_lowercase().as_str()) {
            return Err(format!(
                "Invalid direction '{}'. Valid directions are: asc, desc",
                dir_str
            ));
        }
    }
    
    Ok(())
}

fn validate_deduplicate_params(params: &std::collections::HashMap<String, serde_json::Value>) -> Result<(), String> {
    if let Some(fields) = params.get("fields") {
        if !fields.is_array() {
            return Err("'fields' parameter must be an array".to_string());
        }
        
        let fields_array = fields.as_array().unwrap();
        for field in fields_array {
            if !field.is_string() {
                return Err("All elements in 'fields' array must be strings".to_string());
            }
        }
    }
    
    Ok(())
}

pub fn validate_field_name(field_name: &str) -> Result<(), String> {
    if field_name.is_empty() {
        return Err("Field name cannot be empty".to_string());
    }
    
    if field_name.len() > 100 {
        return Err("Field name cannot exceed 100 characters".to_string());
    }
    
    let field_regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
    if !field_regex.is_match(field_name) {
        return Err(format!(
            "Invalid field name '{}'. Field names must start with a letter or underscore and contain only letters, numbers, and underscores",
            field_name
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::models::request::ProcessRequest;
    
    #[test]
    fn test_validate_empty_data() {
        let req = ProcessRequest::new(vec![], "normalize".to_string(), None);
        let result = validate_request(&req);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_invalid_transformation() {
        let mut data = HashMap::new();
        data.insert("value".to_string(), serde_json::json!(10));
        let req = ProcessRequest::new(vec![data], "invalid_type".to_string(), None);
        let result = validate_request(&req);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_normalize_missing_field() {
        let mut data = HashMap::new();
        data.insert("value".to_string(), serde_json::json!(10));
        let params = HashMap::new();
        let req = ProcessRequest::new(vec![data], "normalize".to_string(), Some(params));
        let result = validate_request(&req);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_field_name_valid() {
        assert!(validate_field_name("valid_field").is_ok());
        assert!(validate_field_name("_private").is_ok());
        assert!(validate_field_name("field123").is_ok());
    }
    
    #[test]
    fn test_validate_field_name_invalid() {
        assert!(validate_field_name("").is_err());
        assert!(validate_field_name("123field").is_err());
        assert!(validate_field_name("field-name").is_err());
    }
}
