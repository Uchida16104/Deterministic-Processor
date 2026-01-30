using System;

namespace DeterministicProcessor;

public static class Processor
{
    public static ProcessorOutput Execute(ProcessorInput input)
    {
        double value = input.Value;

        value = MathTransforms.Apply(value, input.Transform);
        value = RuleEngine.Evaluate(value, input.Rule);

        return new ProcessorOutput
        {
            Result = value
        };
    }
}

