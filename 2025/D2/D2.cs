using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;

using AOC;

using DataSet = AOC.DataFull;
//using DataSet = AOC.DataTest;

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

bool IsRepeatedDigits(Int64 number)
{
    var digitCount = DigitCount(number);
    for (int c = digitCount / 2; c >= 1; c--)
    {
        if (digitCount % c != 0) continue;
        var test = number;
        var power = (Int64)Math.Pow(10, c);
        var pattern = test % power;
        while (test > 0)
        {
            if (test % power != pattern)
            {
                break;
            }
            test /= power;
        }
        if (test == 0)
        {
            return true;
        }
    }
    return false;
}

void TestIsRepeatedDigits()
{
    Debug.Assert(IsRepeatedDigits(12) == false);
    Debug.Assert(IsRepeatedDigits(11) == true);
    Debug.Assert(IsRepeatedDigits(111) == true);
    Debug.Assert(IsRepeatedDigits(1212) == true);
    Debug.Assert(IsRepeatedDigits(123123) == true);
    Debug.Assert(IsRepeatedDigits(12341234) == true);
    Debug.Assert(IsRepeatedDigits(123123123) == true);
    Debug.Assert(IsRepeatedDigits(123412341234) == true);
    Debug.Assert(IsRepeatedDigits(1188511885) == true);
    Debug.Assert(IsRepeatedDigits(446446) == true);
    Debug.Assert(IsRepeatedDigits(38593859) == true);
    Debug.Assert(IsRepeatedDigits(824824824) == true);
    Debug.Assert(IsRepeatedDigits(1231234) == false);
    Debug.Assert(IsRepeatedDigits(1234123) == false);
    Debug.Assert(IsRepeatedDigits(12312312) == false);
    Debug.Assert(IsRepeatedDigits(12341234123) == false);
}

void Part2(string filename)
{
    Int64 total = 0;
    var lines = File.ReadAllLines(filename);
    Debug.Assert(lines.Length == 1);
    foreach (var move in lines[0].Split(','))
    {
        (Int64 low, Int64 high) = SplitRange(move);
        //Util.Log($"{low} to {high} : diff={high-low} : ");
        for (Int64 test = low; test <= high; test++)
        {
            if (IsRepeatedDigits(test))
            {
                //Util.Log($"{test},");
                total += test;
            }
        }
        //Util.LogLine($"  -> Total: {total}");
    }
    Util.LogLine($"Total: {total}");
}

void Run()
{
    //TestIsRepeatedDigits();
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

//D2: Part1: Total: 54234399924
//completed in 20ms
//D2: Part2: Total: 70187097315
//completed in 370ms