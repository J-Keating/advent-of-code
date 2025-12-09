using System.Data;
using System.Diagnostics;
using System.Numerics;
using System.Reflection;

using AOC_Util;
//using DataSet = AOC_Util.DataFull;
using DataSet = AOC_Util.DataTest;

using SegmentDesc = (int from, int to, float distance);

static Vector3[] LoadPoints(string filename)
{
    Vector3 Vec3FromString(string s)
    {
        var parts = s.Split(',').Select(p => Int64.Parse(p.Trim())).ToArray();
        return new Vector3(parts[0], parts[1], parts[2]);
    }
    var lines = File.ReadAllLines(filename);
    return lines.Select(l => Vec3FromString(l)).ToArray();
}

void Part1(string filename)
{
    var points = LoadPoints(filename);
    var sortedSegments = new List<SegmentDesc>((points.Length * (points.Length + 1)) / 2);
    for (int i = 0; i < points.Length; i++)
    {
        for (int j = i+1; j < points.Length; j++)
        {
            sortedSegments.Add(new SegmentDesc { from = i, to = j, distance = Vector3.DistanceSquared(points[i], points[j]) });
        }
    }
    sortedSegments.Sort((a, b) => a.distance.CompareTo(b.distance));

    LogUtil.LogLine($"{sortedSegments[0]},  {points[sortedSegments[0].from]} -> {points[sortedSegments[0].to]}");
    LogUtil.LogLine($"{sortedSegments[1]},  {points[sortedSegments[1].from]} -> {points[sortedSegments[1].to]}");
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

//D7: Part1: 1678
//completed in 6ms
//D7: Part2: 357525737893560
//completed in 6ms