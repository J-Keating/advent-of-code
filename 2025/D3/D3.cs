using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;

using AOC;

using DataSet = AOC.DataFull;
//using DataSet = AOC.DataTest;

void Part1(string filename)
{
    Int64 total = 0;
    var lines = File.ReadAllLines(filename);
    foreach (var line in lines)
    {
        int d1 = -1, d2 = -1;
        int max = 0;
        for (int i = 0; i < line.Length - 1; i++)
        {
            int v = (int)(line[i] - '0');
            if (v > max)
            {
                max = v;
                d1 = i;
            }
        }
        max = 0;
        for (int i = d1 + 1; i < line.Length; i++)
        {
            int v = (int)(line[i] - '0');
            if (v > max)
            {
                max = v;
                d2 = i;
            }
        }
        var joltage = 10 * (line[d1] - '0') + (line[d2] - '0');
        //Util.LogLine($"Line: {line}  D1: {d1}  D2: {d2}  Joltage: {joltage}");
        total += joltage;
    }
    Util.LogLine($"Total: {total}");
}

Int64 Voltage(char[] chars, bool[] flags, int test_pos)
{
    Int64 voltage = 0;
    //for (int i = 0; i < chars.Length; i++)
    //{
    //    if (flags[i] || i == test_pos)
    //    {
    //        voltage = voltage * 10 + (chars[i] - '0');
    //    }
    //}
    foreach(var tuple in chars.Zip(flags, (ch, flag) => (ch, flag))
        .Select((pair, i) => (pair.ch, pair.flag, i))
        .Where(t => t.flag || t.i == test_pos ))
    {
        voltage = voltage * 10 + (tuple.ch - '0');
    }
    return voltage;
}

void Part2(string filename)
{
    Int64 total = 0;
    var lines = File.ReadAllLines(filename);
    foreach (var line in lines)
    {
        var chars = line.ToCharArray();
        var flags = new bool[chars.Length];
        for (int c = 1; c <= 12; c++)
        {
            Int64 max_voltage = -1;
            int max_index = -1;
            for (int i = 0; i < chars.Length; i++)
            {
                var voltage = Voltage(chars, flags, i);
                if (voltage > max_voltage)
                {
                    max_voltage = voltage;
                    max_index = i;
                }
            }
            flags[max_index] = true;
        }
        var voltage_final = Voltage(chars, flags, -1);
        total += voltage_final;
        //Util.LogLine($"Line: {line}  Voltage: {voltage_final}");
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

//D3: Part1: Total: 17405
//completed in 3ms
//D3: Part2: Total: 171990312704598
//completed in 66ms