using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;

namespace DeterministicProcessor
{
    public static class MathTransforms
    {
        public static List<Dictionary<string, object>> Normalize(
            List<Dictionary<string, object>> data,
            Dictionary<string, object>? parameters)
        {
            if (data.Count == 0)
            {
                return new List<Dictionary<string, object>>();
            }
            
            var field = parameters?.ContainsKey("field") == true 
                ? parameters["field"].ToString() 
                : null;
            
            if (string.IsNullOrEmpty(field))
            {
                return data;
            }
            
            var numericValues = new List<double>();
            foreach (var row in data)
            {
                if (row.ContainsKey(field) && row[field] != null)
                {
                    if (double.TryParse(row[field].ToString(), out double value))
                    {
                        numericValues.Add(value);
                    }
                }
            }
            
            if (numericValues.Count == 0)
            {
                return data;
            }
            
            double min = numericValues.Min();
            double max = numericValues.Max();
            double range = max - min;
            
            if (range == 0)
            {
                var singleValueResult = new List<Dictionary<string, object>>();
                foreach (var row in data)
                {
                    var newRow = new Dictionary<string, object>(row);
                    if (newRow.ContainsKey(field))
                    {
                        newRow[field] = 0.5;
                    }
                    singleValueResult.Add(newRow);
                }
                return singleValueResult;
            }
            
            var result = new List<Dictionary<string, object>>();
            foreach (var row in data)
            {
                var newRow = new Dictionary<string, object>(row);
                if (newRow.ContainsKey(field) && newRow[field] != null)
                {
                    if (double.TryParse(newRow[field].ToString(), out double value))
                    {
                        double normalized = (value - min) / range;
                        newRow[field] = Math.Round(normalized, 6);
                    }
                }
                result.Add(newRow);
            }
            
            return result;
        }
        
        public static List<Dictionary<string, object>> Transform(
            List<Dictionary<string, object>> data,
            Dictionary<string, object>? parameters)
        {
            if (data.Count == 0)
            {
                return new List<Dictionary<string, object>>();
            }
            
            var operation = parameters?.ContainsKey("operation") == true 
                ? parameters["operation"].ToString() 
                : "identity";
            
            var field = parameters?.ContainsKey("field") == true 
                ? parameters["field"].ToString() 
                : null;
            
            if (string.IsNullOrEmpty(field))
            {
                return data;
            }
            
            var result = new List<Dictionary<string, object>>();
            foreach (var row in data)
            {
                var newRow = new Dictionary<string, object>(row);
                if (newRow.ContainsKey(field) && newRow[field] != null)
                {
                    if (double.TryParse(newRow[field].ToString(), out double value))
                    {
                        double transformed = operation?.ToLowerInvariant() switch
                        {
                            "square" => value * value,
                            "sqrt" => value >= 0 ? Math.Sqrt(value) : 0,
                            "log" => value > 0 ? Math.Log(value) : 0,
                            "exp" => Math.Exp(value),
                            "abs" => Math.Abs(value),
                            "negate" => -value,
                            "reciprocal" => value != 0 ? 1.0 / value : 0,
                            _ => value
                        };
                        newRow[field] = Math.Round(transformed, 6);
                    }
                }
                result.Add(newRow);
            }
            
            return result;
        }
        
        public static List<Dictionary<string, object>> ComputeStatistics(
            List<Dictionary<string, object>> data,
            Dictionary<string, object>? parameters)
        {
            if (data.Count == 0)
            {
                return new List<Dictionary<string, object>>();
            }
            
            var field = parameters?.ContainsKey("field") == true 
                ? parameters["field"].ToString() 
                : null;
            
            if (string.IsNullOrEmpty(field))
            {
                return data;
            }
            
            var numericValues = new List<double>();
            foreach (var row in data)
            {
                if (row.ContainsKey(field) && row[field] != null)
                {
                    if (double.TryParse(row[field].ToString(), out double value))
                    {
                        numericValues.Add(value);
                    }
                }
            }
            
            if (numericValues.Count == 0)
            {
                return new List<Dictionary<string, object>>();
            }
            
            numericValues.Sort();
            
            double sum = numericValues.Sum();
            double mean = sum / numericValues.Count;
            double variance = numericValues.Sum(v => Math.Pow(v - mean, 2)) / numericValues.Count;
            double stdDev = Math.Sqrt(variance);
            double median = numericValues.Count % 2 == 0
                ? (numericValues[numericValues.Count / 2 - 1] + numericValues[numericValues.Count / 2]) / 2.0
                : numericValues[numericValues.Count / 2];
            
            var stats = new Dictionary<string, object>
            {
                ["field"] = field,
                ["count"] = numericValues.Count,
                ["sum"] = Math.Round(sum, 6),
                ["mean"] = Math.Round(mean, 6),
                ["median"] = Math.Round(median, 6),
                ["min"] = Math.Round(numericValues.Min(), 6),
                ["max"] = Math.Round(numericValues.Max(), 6),
                ["variance"] = Math.Round(variance, 6),
                ["stdDev"] = Math.Round(stdDev, 6)
            };
            
            return new List<Dictionary<string, object>> { stats };
        }
    }
}
