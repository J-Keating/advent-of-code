using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;

using AOC;

//using DataSet = AOC.DataFull;
using DataSet = AOC.DataTest;

(Int64, Int64) SplitRange(string range)
{
    var parts = range.Split('-');
    Debug.Assert(parts.Length == 2);
    return (Int64.Parse(parts[0]), Int64.Parse(parts[1]));
}
int DigitCount(Int64 number)
{
    int count = 0;
    do
    {
        count++;
        number /= 10;
    } while (number > 0);
    return count;
}

Int64 RemoveLowDigits(Int64 number, Int64 digits)
{
    double divisor = Math.Pow(10, digits);
    return (Int64)(number / divisor);
}

void Part1(string filename)
{
    Int64 total = 0;
    var lines = File.ReadAllLines(filename);
    Debug.Assert(lines.Length == 1);
    foreach (var move in lines[0].Split(','))
    {
        (Int64 low, Int64 high) = SplitRange(move);
        //Util.Log($"{low} to {high}  :");
        int lowDigits = DigitCount(low);
        int highDigits = DigitCount(high);
        if (lowDigits % 2 == 1 && highDigits % 2 == 1)
        {
            //Util.LogLine("  -> Odd digit counts only, skipping");
            continue;
        }
        Debug.Assert(highDigits - lowDigits <= 1);
        int digits = (lowDigits % 2 == 0) ? lowDigits : highDigits;
        Debug.Assert(digits % 2 == 0);
        int halfDigits = digits / 2;
        Int64 lowFirst = RemoveLowDigits(low, halfDigits);
        Int64 highFirst = RemoveLowDigits(high, halfDigits);
        for (Int64 first = lowFirst; first <= highFirst; first++)
        {
            Int64 test = first * (Int64)Math.Pow(10, halfDigits) + first;
            if (DigitCount(test) != digits)
            {
                continue;
            }
            if (test >= low && test <= high)
            {
                //Util.Log($"{test},");
                total += test;
            }
        }
        //Util.LogLine($"  -> Total: {total}");
    }
    Util.LogLine($"Total: {total}");
}

void Part2(string filename)
{
    Int64 total = 0;
    var lines = File.ReadAllLines(filename);
    Debug.Assert(lines.Length == 1);
    foreach (var move in lines[0].Split(','))
    {
        (Int64 low, Int64 high) = SplitRange(move);
        Util.Log($"{low} to {high}  :");
        int lowDigits = DigitCount(low);
        int highDigits = DigitCount(high);
        if (lowDigits % 2 == 1 && highDigits % 2 == 1)
        {
            Util.LogLine("  -> Odd digit counts only, skipping");
            continue;
        }
        Debug.Assert(highDigits - lowDigits <= 1);
        int digits = (lowDigits % 2 == 0) ? lowDigits : highDigits;
        Debug.Assert(digits % 2 == 0);
        int halfDigits = digits / 2;
        Int64 lowFirst = RemoveLowDigits(low, halfDigits);
        Int64 highFirst = RemoveLowDigits(high, halfDigits);
        for (Int64 first = lowFirst; first <= highFirst; first++)
        {
            Int64 test = first * (Int64)Math.Pow(10, halfDigits) + first;
            if (DigitCount(test) != digits)
            {
                continue;
            }
            if (test >= low && test <= high)
            {
                Util.Log($"{test},");
                total += test;
            }
        }
        Util.LogLine($"  -> Total: {total}");
    }
    Util.LogLine($"Total: {total}");
}

void Run()
{
    Util.Log($"{Config.Name}: Part1: ");
    Util.Time(() => Part1(DataSet.Filename));
    Util.Log($"{Config.Name}: Part2: ");
    Util.Time(() => Part2(DataSet.Filename));
}

Run();
public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
}

//D2: Part1: Total: 1227775554
//completed in 4480ms