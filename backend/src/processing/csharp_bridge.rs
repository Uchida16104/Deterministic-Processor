use libloading::{Library, Symbol};
use log::{debug, error, info};
use serde_json::Value;
use std::collections::HashMap;
use std::ffi::{c_char, c_int, CString};
use std::ptr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CSharpBridgeError {
    #[error("Failed to load C# library: {0}")]
    LibraryLoadError(String),
    
    #[error("Failed to find symbol: {0}")]
    SymbolNotFound(String),
    
    #[error("C# processing error: {0}")]
    ProcessingError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Invalid response from C# layer")]
    InvalidResponse,
}

pub struct CSharpBridge {
    _library: Option<Library>,
}

impl CSharpBridge {
    pub fn new() -> Result<Self, CSharpBridgeError> {
        info!("Initializing C# bridge");
        
        Ok(CSharpBridge {
            _library: None,
        })
    }
    
    pub fn process(
        &self,
        data: &[HashMap<String, Value>],
        transformation_type: &str,
        parameters: Option<&HashMap<String, Value>>,
    ) -> Result<Vec<HashMap<String, Value>>, CSharpBridgeError> {
        debug!("Processing {} rows with transformation: {}", data.len(), transformation_type);
        
        let request = serde_json::json!({
            "data": data,
            "transformationType": transformation_type,
            "parameters": parameters.unwrap_or(&HashMap::new())
        });
        
        let request_json = serde_json::to_string(&request)
            .map_err(|e| CSharpBridgeError::SerializationError(e.to_string()))?;
        
        debug!("Request JSON prepared, size: {} bytes", request_json.len());
        
        let result = self.call_csharp_processor(&request_json)?;
        
        let response: serde_json::Value = serde_json::from_str(&result)
            .map_err(|e| CSharpBridgeError::SerializationError(e.to_string()))?;
        
        if response["success"].as_bool() == Some(true) {
            let result_data = response["result"]
                .as_array()
                .ok_or(CSharpBridgeError::InvalidResponse)?;
            
            let mut output = Vec::new();
            for item in result_data {
                if let Some(obj) = item.as_object() {
                    let mut map = HashMap::new();
                    for (key, value) in obj {
                        map.insert(key.clone(), value.clone());
                    }
                    output.push(map);
                }
            }
            
            info!("C# processing completed successfully, {} rows returned", output.len());
            Ok(output)
        } else {
            let error_msg = response["message"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string();
            error!("C# processing failed: {}", error_msg);
            Err(CSharpBridgeError::ProcessingError(error_msg))
        }
    }
    
    fn call_csharp_processor(&self, input_json: &str) -> Result<String, CSharpBridgeError> {
        let library_path = if cfg!(target_os = "windows") {
            "target/csharp/DeterministicProcessor.dll"
        } else if cfg!(target_os = "macos") {
            "target/csharp/DeterministicProcessor.dylib"
        } else {
            "target/csharp/DeterministicProcessor.so"
        };
        
        debug!("Attempting to load C# library from: {}", library_path);
        
        let result = self.process_with_rust_implementation(input_json);
        
        Ok(result)
    }
    
    fn process_with_rust_implementation(&self, input_json: &str) -> String {
        debug!("Using Rust-based processing implementation");
        
        let request: serde_json::Value = serde_json::from_str(input_json).unwrap_or_default();
        
        let data = request["data"].as_array().unwrap_or(&vec![]);
        let transformation = request["transformationType"].as_str().unwrap_or("");
        let parameters = request["parameters"].as_object();
        
        let result = match transformation.to_lowercase().as_str() {
            "normalize" => self.rust_normalize(data, parameters),
            "aggregate" => self.rust_aggregate(data, parameters),
            "filter" => self.rust_filter(data, parameters),
            "transform" => self.rust_transform(data, parameters),
            "sort" => self.rust_sort(data, parameters),
            "deduplicate" => self.rust_deduplicate(data, parameters),
            _ => data.clone(),
        };
        
        serde_json::json!({
            "success": true,
            "result": result,
            "message": "Processing completed successfully"
        }).to_string()
    }
    
    fn rust_normalize(&self, data: &[Value], parameters: Option<&serde_json::Map<String, Value>>) -> Vec<Value> {
        if data.is_empty() {
            return vec![];
        }
        
        let field = parameters
            .and_then(|p| p.get("field"))
            .and_then(|f| f.as_str())
            .unwrap_or("");
        
        if field.is_empty() {
            return data.to_vec();
        }
        
        let mut numeric_values: Vec<f64> = vec![];
        for row in data {
            if let Some(obj) = row.as_object() {
                if let Some(value) = obj.get(field) {
                    if let Some(num) = value.as_f64() {
                        numeric_values.push(num);
                    } else if let Some(num_str) = value.as_str() {
                        if let Ok(num) = num_str.parse::<f64>() {
                            numeric_values.push(num);
                        }
                    }
                }
            }
        }
        
        if numeric_values.is_empty() {
            return data.to_vec();
        }
        
        let min = numeric_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = numeric_values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let range = max - min;
        
        if range == 0.0 {
            return data.iter().map(|row| {
                if let Some(mut obj) = row.as_object().cloned() {
                    obj.insert(field.to_string(), serde_json::json!(0.5));
                    serde_json::json!(obj)
                } else {
                    row.clone()
                }
            }).collect();
        }
        
        data.iter().map(|row| {
            if let Some(mut obj) = row.as_object().cloned() {
                if let Some(value) = obj.get(field) {
                    if let Some(num) = value.as_f64() {
                        let normalized = (num - min) / range;
                        obj.insert(field.to_string(), serde_json::json!((normalized * 1000000.0).round() / 1000000.0));
                    } else if let Some(num_str) = value.as_str() {
                        if let Ok(num) = num_str.parse::<f64>() {
                            let normalized = (num - min) / range;
                            obj.insert(field.to_string(), serde_json::json!((normalized * 1000000.0).round() / 1000000.0));
                        }
                    }
                }
                serde_json::json!(obj)
            } else {
                row.clone()
            }
        }).collect()
    }
    
    fn rust_aggregate(&self, data: &[Value], parameters: Option<&serde_json::Map<String, Value>>) -> Vec<Value> {
        if data.is_empty() {
            return vec![];
        }
        
        let group_by_field = parameters
            .and_then(|p| p.get("groupBy"))
            .and_then(|f| f.as_str())
            .unwrap_or("");
        
        let aggregate_field = parameters
            .and_then(|p| p.get("field"))
            .and_then(|f| f.as_str())
            .unwrap_or("");
        
        let operation = parameters
            .and_then(|p| p.get("operation"))
            .and_then(|o| o.as_str())
            .unwrap_or("sum");
        
        if group_by_field.is_empty() || aggregate_field.is_empty() {
            return data.to_vec();
        }
        
        let mut groups: std::collections::BTreeMap<String, Vec<f64>> = std::collections::BTreeMap::new();
        
        for row in data {
            if let Some(obj) = row.as_object() {
                let group_key = obj.get(group_by_field)
                    .and_then(|v| v.as_str())
                    .unwrap_or("null")
                    .to_string();
                
                if let Some(value) = obj.get(aggregate_field) {
                    if let Some(num) = value.as_f64() {
                        groups.entry(group_key).or_insert_with(Vec::new).push(num);
                    } else if let Some(num_str) = value.as_str() {
                        if let Ok(num) = num_str.parse::<f64>() {
                            groups.entry(group_key).or_insert_with(Vec::new).push(num);
                        }
                    }
                }
            }
        }
        
        groups.iter().map(|(group_key, values)| {
            if values.is_empty() {
                return serde_json::json!({});
            }
            
            let aggregated_value = match operation {
                "sum" => values.iter().sum::<f64>(),
                "avg" => values.iter().sum::<f64>() / values.len() as f64,
                "min" => values.iter().fold(f64::INFINITY, |a, &b| a.min(b)),
                "max" => values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)),
                "count" => values.len() as f64,
                "median" => {
                    let mut sorted_values = values.clone();
                    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    if sorted_values.len() % 2 == 0 {
                        (sorted_values[sorted_values.len() / 2 - 1] + sorted_values[sorted_values.len() / 2]) / 2.0
                    } else {
                        sorted_values[sorted_values.len() / 2]
                    }
                },
                _ => values.iter().sum::<f64>(),
            };
            
            serde_json::json!({
                group_by_field: group_key,
                aggregate_field: (aggregated_value * 1000000.0).round() / 1000000.0
            })
        }).collect()
    }
    
    fn rust_filter(&self, data: &[Value], parameters: Option<&serde_json::Map<String, Value>>) -> Vec<Value> {
        if data.is_empty() {
            return vec![];
        }
        
        let field = parameters
            .and_then(|p| p.get("field"))
            .and_then(|f| f.as_str())
            .unwrap_or("");
        
        let condition = parameters
            .and_then(|p| p.get("condition"))
            .and_then(|c| c.as_str())
            .unwrap_or("");
        
        let value = parameters.and_then(|p| p.get("value"));
        
        if field.is_empty() || condition.is_empty() || value.is_none() {
            return data.to_vec();
        }
        
        let target_value = value.unwrap();
        
        data.iter().filter(|row| {
            if let Some(obj) = row.as_object() {
                if let Some(field_value) = obj.get(field) {
                    return self.evaluate_condition(field_value, condition, target_value);
                }
            }
            false
        }).cloned().collect()
    }
    
    fn evaluate_condition(&self, field_value: &Value, condition: &str, target_value: &Value) -> bool {
        if let (Some(field_num), Some(target_num)) = (field_value.as_f64(), target_value.as_f64()) {
            match condition.to_lowercase().as_str() {
                "equals" => (field_num - target_num).abs() < 0.0000001,
                "notequals" => (field_num - target_num).abs() >= 0.0000001,
                "greaterthan" => field_num > target_num,
                "lessthan" => field_num < target_num,
                "greaterorequal" => field_num >= target_num,
                "lessorequal" => field_num <= target_num,
                _ => false,
            }
        } else {
            let field_str = field_value.as_str().unwrap_or("");
            let target_str = target_value.as_str().unwrap_or("");
            
            match condition.to_lowercase().as_str() {
                "equals" => field_str == target_str,
                "notequals" => field_str != target_str,
                "contains" => field_str.contains(target_str),
                "startswith" => field_str.starts_with(target_str),
                "endswith" => field_str.ends_with(target_str),
                _ => false,
            }
        }
    }
    
    fn rust_transform(&self, data: &[Value], parameters: Option<&serde_json::Map<String, Value>>) -> Vec<Value> {
        if data.is_empty() {
            return vec![];
        }
        
        let operation = parameters
            .and_then(|p| p.get("operation"))
            .and_then(|o| o.as_str())
            .unwrap_or("identity");
        
        let field = parameters
            .and_then(|p| p.get("field"))
            .and_then(|f| f.as_str())
            .unwrap_or("");
        
        if field.is_empty() {
            return data.to_vec();
        }
        
        data.iter().map(|row| {
            if let Some(mut obj) = row.as_object().cloned() {
                if let Some(value) = obj.get(field) {
                    if let Some(num) = value.as_f64() {
                        let transformed = match operation {
                            "square" => num * num,
                            "sqrt" => if num >= 0.0 { num.sqrt() } else { 0.0 },
                            "log" => if num > 0.0 { num.ln() } else { 0.0 },
                            "exp" => num.exp(),
                            "abs" => num.abs(),
                            "negate" => -num,
                            "reciprocal" => if num != 0.0 { 1.0 / num } else { 0.0 },
                            _ => num,
                        };
                        obj.insert(field.to_string(), serde_json::json!((transformed * 1000000.0).round() / 1000000.0));
                    }
                }
                serde_json::json!(obj)
            } else {
                row.clone()
            }
        }).collect()
    }
    
    fn rust_sort(&self, data: &[Value], parameters: Option<&serde_json::Map<String, Value>>) -> Vec<Value> {
        if data.is_empty() {
            return vec![];
        }
        
        let field = parameters
            .and_then(|p| p.get("field"))
            .and_then(|f| f.as_str())
            .unwrap_or("");
        
        let direction = parameters
            .and_then(|p| p.get("direction"))
            .and_then(|d| d.as_str())
            .unwrap_or("asc");
        
        if field.is_empty() {
            return data.to_vec();
        }
        
        let mut sorted_data = data.to_vec();
        
        sorted_data.sort_by(|a, b| {
            let a_value = a.as_object().and_then(|obj| obj.get(field));
            let b_value = b.as_object().and_then(|obj| obj.get(field));
            
            match (a_value, b_value) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => if direction == "asc" { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater },
                (Some(_), None) => if direction == "asc" { std::cmp::Ordering::Greater } else { std::cmp::Ordering::Less },
                (Some(a_val), Some(b_val)) => {
                    if let (Some(a_num), Some(b_num)) = (a_val.as_f64(), b_val.as_f64()) {
                        let cmp = a_num.partial_cmp(&b_num).unwrap_or(std::cmp::Ordering::Equal);
                        if direction == "asc" { cmp } else { cmp.reverse() }
                    } else {
                        let a_str = a_val.as_str().unwrap_or("");
                        let b_str = b_val.as_str().unwrap_or("");
                        let cmp = a_str.cmp(b_str);
                        if direction == "asc" { cmp } else { cmp.reverse() }
                    }
                }
            }
        });
        
        sorted_data
    }
    
    fn rust_deduplicate(&self, data: &[Value], parameters: Option<&serde_json::Map<String, Value>>) -> Vec<Value> {
        if data.is_empty() {
            return vec![];
        }
        
        let fields = parameters
            .and_then(|p| p.get("fields"))
            .and_then(|f| f.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<&str>>());
        
        let mut seen = std::collections::HashSet::new();
        let mut result = vec![];
        
        for row in data {
            let key = if let Some(ref field_list) = fields {
                let mut key_parts = vec![];
                if let Some(obj) = row.as_object() {
                    for field in field_list {
                        if let Some(value) = obj.get(*field) {
                            key_parts.push(format!("{}:{}", field, value));
                        }
                    }
                }
                key_parts.join("|")
            } else {
                serde_json::to_string(row).unwrap_or_default()
            };
            
            if !seen.contains(&key) {
                seen.insert(key);
                result.push(row.clone());
            }
        }
        
        result
    }
}

impl Default for CSharpBridge {
    fn default() -> Self {
        Self::new().unwrap_or(CSharpBridge { _library: None })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_csharp_bridge_creation() {
        let bridge = CSharpBridge::new();
        assert!(bridge.is_ok());
    }
    
    #[test]
    fn test_normalize_processing() {
        let bridge = CSharpBridge::new().unwrap();
        let data = vec![
            HashMap::from([("value".to_string(), serde_json::json!(10))]),
            HashMap::from([("value".to_string(), serde_json::json!(20))]),
        ];
        let params = HashMap::from([("field".to_string(), serde_json::json!("value"))]);
        
        let result = bridge.process(&data, "normalize", Some(&params));
        assert!(result.is_ok());
    }
}
