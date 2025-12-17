using System.Data;
using System.Diagnostics;
using System.Reflection;

using AOC_Util;
using DataSet = AOC_Util.DataFull;
//using DataSet = AOC_Util.DataTest;

using ConnectionList = System.Collections.Generic.Dictionary<string, System.Collections.Generic.List<string>>;

Dictionary<string, List<string>> LoadFile(string filename)
{
    var lines = File.ReadAllLines(filename);
    List<(string inputString, List<string> outputStrings)> linesParsed = lines.Select(l =>
    {
        var splits = l.Split(':');
        Debug.Assert(splits.Length == 2);
        var inputString = splits[0].Trim();
        var outputStrings = splits[1].Trim().Split().ToList();
        return (inputString, outputStrings);
    }).ToList();
    var ret = new Dictionary<string, List<string>>();
    foreach (var line in linesParsed)
    {
        ret.Add(line.inputString, line.outputStrings);
    }
    return ret;
}
int CountPathsFrom(ConnectionList connections, string current, HashSet<string> visited, Func<HashSet<string>, bool> shouldCount)
{
    if (current == Config.endNode)
    {
        return shouldCount(visited) ? 1 : 0;
    }
    int ret = 0;
    var outputs = connections[current];
    foreach (var output in outputs)
    {
        if (!visited.Contains(output))
        {
            visited.Add(output);
            ret += CountPathsFrom(connections, output, visited, shouldCount);
            visited.Remove(output);
        }
    }
    return ret;
}
void Part1(string filename)
{
    var connections = LoadFile(filename);
    var validPaths = CountPathsFrom(connections, "you", new HashSet<string>(), (_) => true);
    LogUtil.LogLine($"{validPaths}");
}

void Part2(string filename)
{
    var connections = LoadFile(filename);
    var validPaths = CountPathsFrom(connections, "svr", new HashSet<string>(), (visited) => visited.Contains("dac") && visited.Contains("fft"));
    LogUtil.LogLine($"{validPaths}");
}

void Run()
{
    LogUtil.Log($"{Config.Name}: Part1: ");
    LogUtil.Time(() => Part1(DataSet.Filename));
    LogUtil.Log($"{Config.Name}: Part2: ");
    LogUtil.Time(() => Part2(DataSet.Filename));
}

Run();
//public class Connections
//{
//    public int input;
//    public required List<int> outputs;
//}

public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
    public static readonly string startNode = "you";
    public static readonly string endNode = "out";
}

//D7: Part1: 1678
//completed in 6ms
//D7: Part2: 357525737893560
//completed in 6ms