using System;

namespace DeterministicProcessor;

public static class MathTransforms
{
    public static double Apply(double value, string transform)
    {
        return transform switch
        {
            "sin" => Math.Sin(value),
            "cos" => Math.Cos(value),
            "tan" => Math.Tan(value),
            "sqrt" => Math.Sqrt(value),
            "square" => value * value,
            _ => value
        };
    }
}
