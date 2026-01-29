using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using System.Runtime.InteropServices;

namespace DeterministicProcessor
{
    public static class DataProcessor
    {
        [UnmanagedCallersOnly(EntryPoint = "process_data")]
        public static IntPtr ProcessData(IntPtr inputPtr, int inputLength)
        {
            try
            {
                byte[] inputBytes = new byte[inputLength];
                Marshal.Copy(inputPtr, inputBytes, 0, inputLength);
                string inputJson = System.Text.Encoding.UTF8.GetString(inputBytes);
                
                var request = JsonSerializer.Deserialize<ProcessingRequest>(inputJson);
                
                if (request == null || request.Data == null || request.TransformationType == null)
                {
                    return CreateErrorResponse("Invalid request: missing required fields");
                }
                
                var result = ApplyTransformation(request.Data, request.TransformationType, request.Parameters);
                
                var response = new ProcessingResponse
                {
                    Success = true,
                    Result = result,
                    Message = "Processing completed successfully"
                };
                
                string responseJson = JsonSerializer.Serialize(response);
                byte[] responseBytes = System.Text.Encoding.UTF8.GetBytes(responseJson);
                
                IntPtr resultPtr = Marshal.AllocHGlobal(responseBytes.Length + 4);
                Marshal.WriteInt32(resultPtr, responseBytes.Length);
                Marshal.Copy(responseBytes, 0, resultPtr + 4, responseBytes.Length);
                
                return resultPtr;
            }
            catch (Exception ex)
            {
                return CreateErrorResponse($"Processing error: {ex.Message}");
            }
        }
        
        private static IntPtr CreateErrorResponse(string errorMessage)
        {
            var response = new ProcessingResponse
            {
                Success = false,
                Result = new List<Dictionary<string, object>>(),
                Message = errorMessage
            };
            
            string responseJson = JsonSerializer.Serialize(response);
            byte[] responseBytes = System.Text.Encoding.UTF8.GetBytes(responseJson);
            
            IntPtr resultPtr = Marshal.AllocHGlobal(responseBytes.Length + 4);
            Marshal.WriteInt32(resultPtr, responseBytes.Length);
            Marshal.Copy(responseBytes, 0, resultPtr + 4, responseBytes.Length);
            
            return resultPtr;
        }
        
        private static List<Dictionary<string, object>> ApplyTransformation(
            List<Dictionary<string, object>> data,
            string transformationType,
            Dictionary<string, object>? parameters)
        {
            var sortedData = data.OrderBy(row => JsonSerializer.Serialize(row)).ToList();
            
            return transformationType.ToLowerInvariant() switch
            {
                "normalize" => MathTransforms.Normalize(sortedData, parameters),
                "aggregate" => RuleEngine.Aggregate(sortedData, parameters),
                "filter" => RuleEngine.Filter(sortedData, parameters),
                "transform" => MathTransforms.Transform(sortedData, parameters),
                "sort" => RuleEngine.Sort(sortedData, parameters),
                "deduplicate" => RuleEngine.Deduplicate(sortedData, parameters),
                _ => sortedData
            };
        }
        
        [UnmanagedCallersOnly(EntryPoint = "free_memory")]
        public static void FreeMemory(IntPtr ptr)
        {
            if (ptr != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(ptr);
            }
        }
    }
    
    public class ProcessingRequest
    {
        public List<Dictionary<string, object>>? Data { get; set; }
        public string? TransformationType { get; set; }
        public Dictionary<string, object>? Parameters { get; set; }
    }
    
    public class ProcessingResponse
    {
        public bool Success { get; set; }
        public List<Dictionary<string, object>>? Result { get; set; }
        public string? Message { get; set; }
    }
}
