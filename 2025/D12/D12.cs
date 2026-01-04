using AOC_Util;
using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Text.RegularExpressions;
using BeamSet = System.Collections.Generic.Dictionary<int, System.Int64>;
using DataSet = AOC_Util.DataFull;
//using DataSet = AOC_Util.DataTest;
using TestCase = (int width, int height, int[] counts);

(char[][][], TestCase[]) LoadFile(string filename)
{
    var fileContents = File.ReadAllText(filename);
    var chunks = fileContents.Split("\r\n\r\n");
    var shapes = chunks[0..^1].Select(c =>
    {
        var lines = c.Split("\r\n");
        var ret = lines[1..].Select(l => l.Trim().ToArray()).ToArray();
        return ret;
    }).ToArray();
    var testCases = chunks[^1].Split("\r\n").Select(l =>
    {
        var tokens = l.Split();
        var match = Regex.Match(tokens[0], @"(\d+)x(\d+):");
        Debug.Assert(match.Success);
        return new TestCase { width = int.Parse(match.Groups[1].Value), height = int.Parse(match.Groups[2].Value), counts = [..tokens[1..].Select(t => int.Parse(t))] };
    }).ToArray();
    return (shapes, testCases);
}

void Part1(string filename)
{
    var (shapes, testCases) = LoadFile(filename);
    int shapeCount = shapes.Length;
    Debug.Assert(shapes[0].Length == 3 && shapes[0][0].Length == 3);
    var shapeSizes = shapes.Select(shape => shape.Sum(row => row.Count(c => c == '#'))).ToArray();
    int trivialFailCount = 0;
    int trivialSuccessCount = 0;
    int otherCount = 0;
    foreach ((var width, var height, var counts) in testCases)
    {
        Debug.Assert(counts.Length == shapes.Length);
        int totalHashCount = Enumerable.Range(0, shapes.Length).Select(i => shapeSizes[i] * counts[i]).Sum();
        int threeSquareCount = (width / 3) * (height / 3);
        if (totalHashCount > width * height)
        {
            trivialFailCount++;
        }
        else if (threeSquareCount >= shapeSizes.Sum())
        {
            trivialSuccessCount++;
        }
        else
        {
            otherCount++;
        }
    }
    LogUtil.LogLine($"Trivial Pass: {trivialSuccessCount}, Trivial Fail: {trivialFailCount}, Other: {otherCount}");
}

void Run()
{
    LogUtil.Log($"{Config.Name}: Part1: ");
    LogUtil.Time(() => Part1(DataSet.Filename));
}

Run();

public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
}

//D12: Part1: Trivial Pass: 410, Trivial Fail: 590, Other: 0
//completed in 32ms