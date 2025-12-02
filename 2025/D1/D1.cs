using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;

using AOC;

using DataSet = AOC.DataFull;
//using DataSet = AOC.DataTest;

IEnumerable<int> LinesToMoves(String[] lines) => lines.Select(line =>
{
    char dirChar = line[0];
    int amount = int.Parse(line[1..]);
    Debug.Assert(dirChar == 'L' || dirChar == 'R');
    return dirChar == 'L' ? -amount : amount;
});

void Part1(string filename)
{
    int zeroCount = 0;
    int current = 50;
    foreach (var move in LinesToMoves(File.ReadAllLines(filename)))
    {
        current += move;
        current = (current + 100) % 100;
        if (current == 0)
        {
            zeroCount++;
        }
    }
    Util.LogLine($"Final position: {current}, zero crossings: {zeroCount}");
}

void Part2(string filename)
{
    int zeroCount = 0;
    int current = 50;
    foreach (var _move in LinesToMoves(File.ReadAllLines(filename)))
    {
        var move = _move;
        Debug.Assert(current >= 0 && current < 100);
        zeroCount += Math.Abs(move / 100);
        move %= 100;
        Debug.Assert(-99 <= move && move <= 99 && move != 0);
        var next = current + move;
        if (current != 0 && (next <= 0) || (next >= 100))
        {
            zeroCount++;
        }
        current = (next + 100) % 100;
    }
    Util.LogLine($"Final position: {current}, zero crossings: {zeroCount}");
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

//D1: Part1: Final position: 11, zero crossings: 1129
//completed in 15ms
//D1: Part2: Final position: 11, zero crossings: 6638
//completed in 1ms