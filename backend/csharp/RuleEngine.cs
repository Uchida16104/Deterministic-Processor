using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using System.Text.RegularExpressions;

namespace DeterministicProcessor
{
    public static class RuleEngine
    {
        public static List<Dictionary<string, object>> Aggregate(
            List<Dictionary<string, object>> data,
            Dictionary<string, object>? parameters)
        {
            if (data.Count == 0)
            {
                return new List<Dictionary<string, object>>();
            }
            
            var groupByField = parameters?.ContainsKey("groupBy") == true 
                ? parameters["groupBy"].ToString() 
                : null;
            
            var aggregateField = parameters?.ContainsKey("field") == true 
                ? parameters["field"].ToString() 
                : null;
            
            var operation = parameters?.ContainsKey("operation") == true 
                ? parameters["operation"].ToString() 
                : "sum";
            
            if (string.IsNullOrEmpty(groupByField) || string.IsNullOrEmpty(aggregateField))
            {
                return data;
            }
            
            var groups = new SortedDictionary<string, List<double>>();
            
            foreach (var row in data)
            {
                if (row.ContainsKey(groupByField) && row.ContainsKey(aggregateField))
                {
                    var groupKey = row[groupByField]?.ToString() ?? "null";
                    
                    if (!groups.ContainsKey(groupKey))
                    {
                        groups[groupKey] = new List<double>();
                    }
                    
                    if (double.TryParse(row[aggregateField]?.ToString(), out double value))
                    {
                        groups[groupKey].Add(value);
                    }
                }
            }
            
            var result = new List<Dictionary<string, object>>();
            foreach (var group in groups.OrderBy(g => g.Key))
            {
                var values = group.Value;
                if (values.Count == 0) continue;
                
                values.Sort();
                
                double aggregatedValue = operation?.ToLowerInvariant() switch
                {
                    "sum" => values.Sum(),
                    "avg" => values.Average(),
                    "min" => values.Min(),
                    "max" => values.Max(),
                    "count" => values.Count,
                    "median" => values.Count % 2 == 0
                        ? (values[values.Count / 2 - 1] + values[values.Count / 2]) / 2.0
                        : values[values.Count / 2],
                    _ => values.Sum()
                };
                
                var resultRow = new Dictionary<string, object>
                {
                    [groupByField] = group.Key,
                    [aggregateField] = Math.Round(aggregatedValue, 6)
                };
                
                result.Add(resultRow);
            }
            
            return result;
        }
        
        public static List<Dictionary<string, object>> Filter(
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
            
            var condition = parameters?.ContainsKey("condition") == true 
                ? parameters["condition"].ToString() 
                : null;
            
            var value = parameters?.ContainsKey("value") == true 
                ? parameters["value"] 
                : null;
            
            if (string.IsNullOrEmpty(field) || string.IsNullOrEmpty(condition) || value == null)
            {
                return data;
            }
            
            var result = new List<Dictionary<string, object>>();
            foreach (var row in data)
            {
                if (row.ContainsKey(field) && row[field] != null)
                {
                    bool matches = EvaluateCondition(row[field], condition, value);
                    if (matches)
                    {
                        result.Add(new Dictionary<string, object>(row));
                    }
                }
            }
            
            return result;
        }
        
        private static bool EvaluateCondition(object fieldValue, string condition, object targetValue)
        {
            string fieldStr = fieldValue?.ToString() ?? "";
            string targetStr = targetValue?.ToString() ?? "";
            
            if (double.TryParse(fieldStr, out double fieldNum) && 
                double.TryParse(targetStr, out double targetNum))
            {
                return condition.ToLowerInvariant() switch
                {
                    "equals" => Math.Abs(fieldNum - targetNum) < 0.0000001,
                    "notequals" => Math.Abs(fieldNum - targetNum) >= 0.0000001,
                    "greaterthan" => fieldNum > targetNum,
                    "lessthan" => fieldNum < targetNum,
                    "greaterorequal" => fieldNum >= targetNum,
                    "lessorequal" => fieldNum <= targetNum,
                    _ => false
                };
            }
            
            return condition.ToLowerInvariant() switch
            {
                "equals" => string.Equals(fieldStr, targetStr, StringComparison.Ordinal),
                "notequals" => !string.Equals(fieldStr, targetStr, StringComparison.Ordinal),
                "contains" => fieldStr.Contains(targetStr, StringComparison.Ordinal),
                "startswith" => fieldStr.StartsWith(targetStr, StringComparison.Ordinal),
                "endswith" => fieldStr.EndsWith(targetStr, StringComparison.Ordinal),
                "matches" => IsRegexMatch(fieldStr, targetStr),
                _ => false
            };
        }
        
        private static bool IsRegexMatch(string input, string pattern)
        {
            try
            {
                return Regex.IsMatch(input, pattern, RegexOptions.None, TimeSpan.FromMilliseconds(100));
            }
            catch
            {
                return false;
            }
        }
        
        public static List<Dictionary<string, object>> Sort(
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
            
            var direction = parameters?.ContainsKey("direction") == true 
                ? parameters["direction"].ToString() 
                : "asc";
            
            if (string.IsNullOrEmpty(field))
            {
                return data;
            }
            
            var sortedData = data.Select(row => new Dictionary<string, object>(row)).ToList();
            
            sortedData.Sort((a, b) =>
            {
                object? aValue = a.ContainsKey(field) ? a[field] : null;
                object? bValue = b.ContainsKey(field) ? b[field] : null;
                
                if (aValue == null && bValue == null) return 0;
                if (aValue == null) return direction == "asc" ? -1 : 1;
                if (bValue == null) return direction == "asc" ? 1 : -1;
                
                string aStr = aValue.ToString() ?? "";
                string bStr = bValue.ToString() ?? "";
                
                if (double.TryParse(aStr, out double aNum) && double.TryParse(bStr, out double bNum))
                {
                    int numComparison = aNum.CompareTo(bNum);
                    return direction == "asc" ? numComparison : -numComparison;
                }
                
                int strComparison = string.Compare(aStr, bStr, StringComparison.Ordinal);
                return direction == "asc" ? strComparison : -strComparison;
            });
            
            return sortedData;
        }
        
        public static List<Dictionary<string, object>> Deduplicate(
            List<Dictionary<string, object>> data,
            Dictionary<string, object>? parameters)
        {
            if (data.Count == 0)
            {
                return new List<Dictionary<string, object>>();
            }
            
            var fields = parameters?.ContainsKey("fields") == true 
                ? (parameters["fields"] as List<object>)?.Select(f => f.ToString()).ToList()
                : null;
            
            var seen = new HashSet<string>();
            var result = new List<Dictionary<string, object>>();
            
            foreach (var row in data)
            {
                string key;
                if (fields != null && fields.Count > 0)
                {
                    var keyParts = new List<string>();
                    foreach (var field in fields.OrderBy(f => f))
                    {
                        if (field != null && row.ContainsKey(field))
                        {
                            keyParts.Add($"{field}:{row[field]}");
                        }
                    }
                    key = string.Join("|", keyParts);
                }
                else
                {
                    key = JsonSerializer.Serialize(row);
                }
                
                if (!seen.Contains(key))
                {
                    seen.Add(key);
                    result.Add(new Dictionary<string, object>(row));
                }
            }
            
            return result;
        }
    }
}
