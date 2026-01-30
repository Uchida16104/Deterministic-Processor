namespace DeterministicProcessor;

public sealed class ProcessorInput
{
    public double Value { get; init; }
    public string Transform { get; init; } = "";
    public string Rule { get; init; } = "";
}

public sealed class ProcessorOutput
{
    public double Result { get; init; }
}

