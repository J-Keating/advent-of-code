using AOC_Util;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Reflection;

using ConnectionList = System.Collections.Generic.Dictionary<string, System.Collections.Generic.List<string>>;

//using DataSet = AOC_Util.DataTest;
using DataSet = AOC_Util.DataFull;

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

Int64 CountPathsFrom(ConnectionList connections, string current, string endNode, HashSet<string> visited, Dictionary<string, Int64> cache)
{
    if (current == endNode)
    {
        return 1;
    }
    Int64 ret = 0;
    if (!cache.TryGetValue(current, out ret))
    {
        var outputs = connections[current];
        foreach (var output in outputs)
        {
            //Debug.Assert(!visited.Contains(output));
            //if (!visited.Contains(output))
            {
                //visited.Add(output);
                ret += CountPathsFrom(connections, output, endNode, visited, cache);
                //visited.Remove(output);
            }
        }
        cache.Add(current, ret);
    }
    return ret;
}

void Part1(string filename)
{
    var connections = LoadFile(filename);
    var validPaths = CountPathsFrom(connections, "you", "out", new HashSet<string>(), new Dictionary<string, Int64>());
    LogUtil.LogLine($"{validPaths}");
}

void Part2(string filename)
{
    var connections = LoadFile(filename);
    connections.Add("out", new List<string>());
    var dacThenFft =
        CountPathsFrom(connections, "svr", "dac", new HashSet<string>(), new Dictionary<string, Int64>()) *
        CountPathsFrom(connections, "dac", "fft", new HashSet<string>(), new Dictionary<string, Int64>()) *
        CountPathsFrom(connections, "fft", "out", new HashSet<string>(), new Dictionary<string, Int64>());
    var fftThenDac=
        CountPathsFrom(connections, "svr", "fft", new HashSet<string>(), new Dictionary<string, Int64>()) *
        CountPathsFrom(connections, "fft", "dac", new HashSet<string>(), new Dictionary<string, Int64>()) *
        CountPathsFrom(connections, "dac", "out", new HashSet<string>(), new Dictionary<string, Int64>());
    LogUtil.LogLine($"{dacThenFft + fftThenDac}");
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

//D11: Part1: 497
//completed in 5ms
//D11: Part2: 358564784931864
//completed in 2ms