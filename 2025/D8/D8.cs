using System.Data;
using System.Numerics;
using System.Reflection;

using AOC_Util;

//using DataSet = AOC_Util.DataFull;
//using DataSetProblem = DataFullProblem;
using DataSet = AOC_Util.DataTest;
using DataSetProblem = DataTestProblem;

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

    var circuits = new List<HashSet<int>>();
    foreach (var segment in sortedSegments[0..DataSetProblem.ConnectionCount])
    {
        var touchedCircuits = circuits.Where((c) => c.Contains(segment.from) || c.Contains(segment.to)).ToArray();
        switch (touchedCircuits.Length)
        {
            case 0:
                circuits.Add(new HashSet<int>([ segment.from, segment.to ] ));
                break;
            case 1:
                touchedCircuits[0].Add(segment.from);
                touchedCircuits[0].Add(segment.to);
                break;
            case 2:
                circuits.Remove(touchedCircuits[0]);
                circuits.Remove(touchedCircuits[1]);
                circuits.Add(new HashSet<int>(Enumerable.Union(touchedCircuits[0], touchedCircuits[1])));
                break;
            default:
                throw new InvalidDataException();
        }
    }
    var sortedCircuitSizes = circuits.Select(c => c.Count).ToList();
    sortedCircuitSizes.Sort((a, b) => b.CompareTo(a));
    var result = sortedCircuitSizes.Take(3).Aggregate(1, (a, s) => a * s);
    LogUtil.LogLine($"{result}");
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

static class DataTestProblem
{
    public static readonly int ConnectionCount = 10;
}
static class DataFullProblem
{
    public static readonly int ConnectionCount = 1000;
}

public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
}

//D7: Part1: 1678
//completed in 6ms
//D7: Part2: 357525737893560
//completed in 6ms