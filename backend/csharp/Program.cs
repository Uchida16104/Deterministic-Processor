using System;
using System.IO;
using System.Text;
using System.Text.Json;

namespace DeterministicProcessor;

public static class Program
{
    public static int Main(string[] args)
    {
        try
        {
            string inputJson;
            using (var reader = new StreamReader(Console.OpenStandardInput(), Encoding.UTF8))
            {
                inputJson = reader.ReadToEnd();
            }

            if (string.IsNullOrWhiteSpace(inputJson))
                throw new InvalidOperationException("Input JSON is empty.");

            var input = JsonSerializer.Deserialize<ProcessorInput>(inputJson)
                        ?? throw new InvalidOperationException("Invalid JSON.");

            var output = Processor.Execute(input);

            var json = JsonSerializer.Serialize(output);
            Console.Out.Write(json);
            return 0;
        }
        catch (Exception ex)
        {
            Console.Error.WriteLine(ex.Message);
            return 1;
        }
    }
}
