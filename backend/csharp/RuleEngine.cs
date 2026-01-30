namespace DeterministicProcessor;

public static class RuleEngine
{
    public static double Evaluate(double value, string rule)
    {
        return rule switch
        {
            "clamp_0_1" => Clamp(value, 0.0, 1.0),
            "abs" => System.Math.Abs(value),
            _ => value
        };
    }

    private static double Clamp(double v, double min, double max)
    {
        if (v < min) return min;
        if (v > max) return max;
        return v;
    }
}
