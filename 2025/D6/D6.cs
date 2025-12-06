using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using AOC;

using DataSet = AOC.DataFull;
//using DataSet = AOC.DataTest;

void Part1(string filename)
{
    var lines = File.ReadAllLines(filename);
    var rows = lines[0..^1].Select(l => l.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(n => Int64.Parse(n)).ToArray()).ToArray();
    var ops = lines[^1].Split(' ', StringSplitOptions.RemoveEmptyEntries).ToArray();
    foreach (var row in rows)
    {
        Debug.Assert(row.Length == ops.Length);
    }
    Int64 final = 0;
    Int64 current = 0;
    for (int i = 0; i < ops.Length; i++)
    {
        switch (ops[i]) {
            case "+":
                current = 0;
                for (int j = 0; j < rows.Length; j++)
                {
                    current += rows[j][i];
                }
                break;
            case "*":
                current = 1;
                for (int j = 0; j < rows.Length; j++)
                {
                    current *= rows[j][i];
                }
                break;
            default:
                throw new InvalidOperationException($"Unknown operator: {ops[i]}");

        }
        final += current;
    }
    LogUtil.LogLine($"{final}");
}


void Part2(string filename)
{
    LogUtil.LogLine($"{filename}");
}

void Run()
{
    LogUtil.Log($"{Config.Name}: Part1: ");
    LogUtil.Time(() => Part1(DataSet.Filename));
    LogUtil.Log($"{Config.Name}: Part2: ");
    LogUtil.Time(() => Part2(DataSet.Filename));
}

Run();

enum EdgeType {
    Start = 1,
    Stop = -1
};
class Edge {
    public Int64 val;
    public EdgeType type;
};
public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
}

//D5: Part1: 509
//completed in 22ms
//D5: Part2: 336790092076620
//completed in 44ms