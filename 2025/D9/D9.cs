using System.Data;
using System.Diagnostics;
using System.Reflection;

using AOC_Util;
using DataSet = AOC_Util.DataFull;
//using DataSet = AOC_Util.DataTest;

(Int64 row, Int64 col)[] LoadFile(string filename)
{
    var lines = File.ReadAllLines(filename);
    (Int64 row, Int64 col)[] locs = lines.Select(l =>
    {
        var split = l.Split(',');
        Debug.Assert(split.Length == 2);
        return (Int64.Parse(split[1]), Int64.Parse(split[0]));
    }).ToArray();
    return locs;
}

void Part1(string filename)
{
    (Int64 row, Int64 col)[] locs = LoadFile(filename);
    Int64 maxArea = 0;
    for (int i = 0; i < locs.Length; i++)
    {
        for (int j = 0; j < locs.Length; j++)
        {
            var p1 = locs[i];
            var p2 = locs[j];
            var area = ((Math.Abs(p1.row - p2.row) + 1) * (Math.Abs(p1.col - p2.col) + 1));
            maxArea = Math.Max(maxArea, area);
        }
    }
    LogUtil.LogLine($"{maxArea}");
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

public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
}

//D9: Part1: 4781377701
//completed in 6ms
//D9: Part2: input.txt
//completed in 0ms