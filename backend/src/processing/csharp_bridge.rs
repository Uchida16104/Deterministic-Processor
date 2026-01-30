use log::{debug, info};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CSharpBridgeError {
    #[error("Processing error: {0}")]
    ProcessingError(String),

    #[error("Invalid response")]
    InvalidResponse,
}

pub struct CSharpBridge;

impl CSharpBridge {
    pub fn new() -> Result<Self, CSharpBridgeError> {
        info!("Initializing deterministic Rust processor (C# bridge stub)");
        Ok(Self)
    }

    pub fn process(
        &self,
        data: &[HashMap<String, Value>],
        transformation_type: &str,
        parameters: Option<&HashMap<String, Value>>,
    ) -> Result<Vec<HashMap<String, Value>>, CSharpBridgeError> {
        debug!(
            "Processing {} rows with transformation: {}",
            data.len(),
            transformation_type
        );

        let json_data: Vec<Value> = data
            .iter()
            .map(|row| Value::Object(row.iter().map(|(k, v)| (k.clone(), v.clone())).collect()))
            .collect();

        let param_map = parameters.cloned().unwrap_or_default();

        let processed = self.process_internal(&json_data, transformation_type, &param_map);

        let output = processed
            .into_iter()
            .filter_map(|v| v.as_object().cloned())
            .map(|m| m.into_iter().collect())
            .collect();

        Ok(output)
    }

    fn process_internal(
        &self,
        data: &[Value],
        transformation: &str,
        params: &HashMap<String, Value>,
    ) -> Vec<Value> {
        match transformation.to_lowercase().as_str() {
            "normalize" => self.normalize(data, params),
            "aggregate" => self.aggregate(data, params),
            "filter" => self.filter(data, params),
            "transform" => self.transform(data, params),
            "sort" => self.sort(data, params),
            "deduplicate" => self.deduplicate(data),
            _ => data.to_vec(),
        }
    }

    fn normalize(&self, data: &[Value], params: &HashMap<String, Value>) -> Vec<Value> {
        let field = params.get("field").and_then(|v| v.as_str()).unwrap_or("");
        if field.is_empty() {
            return data.to_vec();
        }

        let nums: Vec<f64> = data
            .iter()
            .filter_map(|row| row.get(field)?.as_f64())
            .collect();

        if nums.is_empty() {
            return data.to_vec();
        }

        let min = nums.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = max - min;

        data.iter()
            .map(|row| {
                let mut obj = row.as_object().cloned().unwrap_or_default();
                if let Some(v) = obj.get(field).and_then(|v| v.as_f64()) {
                    let norm = if range == 0.0 { 0.5 } else { (v - min) / range };
                    obj.insert(field.to_string(), Value::from(norm));
                }
                Value::Object(obj)
            })
            .collect()
    }

    fn aggregate(&self, data: &[Value], params: &HashMap<String, Value>) -> Vec<Value> {
        let group = params.get("groupBy").and_then(|v| v.as_str()).unwrap_or("");
        let field = params.get("field").and_then(|v| v.as_str()).unwrap_or("");

        if group.is_empty() || field.is_empty() {
            return data.to_vec();
        }

        let mut map: HashMap<String, Vec<f64>> = HashMap::new();

        for row in data {
            if let (Some(g), Some(v)) = (
                row.get(group).and_then(|v| v.as_str()),
                row.get(field).and_then(|v| v.as_f64()),
            ) {
                map.entry(g.to_string()).or_default().push(v);
            }
        }

        map.into_iter()
            .map(|(k, vals)| {
                let sum: f64 = vals.iter().sum();
                serde_json::json!({ group: k, field: sum })
            })
            .collect()
    }

    fn filter(&self, data: &[Value], params: &HashMap<String, Value>) -> Vec<Value> {
        let field = params.get("field").and_then(|v| v.as_str()).unwrap_or("");
        let value = params.get("value");

        if field.is_empty() || value.is_none() {
            return data.to_vec();
        }

        data.iter()
            .filter(|row| row.get(field) == value)
            .cloned()
            .collect()
    }

    fn transform(&self, data: &[Value], params: &HashMap<String, Value>) -> Vec<Value> {
        let field = params.get("field").and_then(|v| v.as_str()).unwrap_or("");
        if field.is_empty() {
            return data.to_vec();
        }

        data.iter()
            .map(|row| {
                let mut obj = row.as_object().cloned().unwrap_or_default();
                if let Some(v) = obj.get(field).and_then(|v| v.as_f64()) {
                    obj.insert(field.to_string(), Value::from(v * v));
                }
                Value::Object(obj)
            })
            .collect()
    }

    fn sort(&self, data: &[Value], params: &HashMap<String, Value>) -> Vec<Value> {
        let field = params.get("field").and_then(|v| v.as_str()).unwrap_or("");
        let mut out = data.to_vec();

        out.sort_by(|a, b| {
            a.get(field)
                .and_then(|v| v.as_f64())
                .partial_cmp(&b.get(field).and_then(|v| v.as_f64()))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        out
    }

    fn deduplicate(&self, data: &[Value]) -> Vec<Value> {
        let mut seen = HashSet::new();
        let mut out = vec![];

        for row in data {
            let key = serde_json::to_string(row).unwrap_or_default();
            if seen.insert(key) {
                out.push(row.clone());
            }
        }

        out
    }
}

impl Default for CSharpBridge {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
