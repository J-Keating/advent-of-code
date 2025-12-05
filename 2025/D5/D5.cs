using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using AOC;

using DataSet = AOC.DataFull;
//using DataSet = AOC.DataTest;

static (List<(Int64, Int64)>, List<Int64>) LoadRangesAndNumbers(string filename)
{
    var text = File.ReadAllText(filename);
    (var part1, var part2) = text.Split("\r\n\r\n") switch
    {
        var parts when parts.Length == 2 => (parts[0], parts[1]),
        _ => throw new InvalidDataException("Expected exactly two parts separated by a blank line")
    };
    var ranges = part1.Split('\n')
        .Where(line => !string.IsNullOrWhiteSpace(line))
        .Select(line =>
        {
            var tokens = line.Split('-');
            Debug.Assert(tokens.Length == 2);
            return (Min: Int64.Parse(tokens[0]), Max: Int64.Parse(tokens[1]));
        })
        .ToList();
    var numbers = part2.Split('\n')
        .Where(line => !string.IsNullOrWhiteSpace(line))
        .Select(line => Int64.Parse(line))
        .ToList();
    return (ranges, numbers);
}

bool NumInAnyRange(Int64 num, List<(Int64 min, Int64 max)> ranges)
{
    return ranges.Any(range => range.min <= num && num <= range.max);
}

void Part1(string filename)
{
    (var ranges, var numbers) = LoadRangesAndNumbers(filename);
    var freshCount = numbers.Where(num => NumInAnyRange(num, ranges)).Count();
    LogUtil.LogLine($"{freshCount}");
}


void Part2(string filename)
{
    (var ranges, _) = LoadRangesAndNumbers(filename);
    var edges = new Edge[ranges.Count() * 2];
    var i = 0;
    foreach (var range in ranges)
    {
        edges[i++] = new Edge { val = range.Item1, type = EdgeType.Start };
        edges[i++] = new Edge { val = range.Item2, type = EdgeType.Stop };
    }
    edges.Sort<Edge>((a, b) =>
    {
        var cmp = a.val.CompareTo(b.val);
        if (cmp != 0) return cmp;
        return b.type.CompareTo(a.type); // Purposely reversed to sort Starts before Stops for the same val
    });
    Int64 ingredient_count = 0;
    var depth = 0;
    for (int e = 0; e < edges.Length - 1; e++)
    {
        depth += (int)edges[e].type;
        Debug.Assert(depth >= 0);
        if (depth > 0)
        {
            ingredient_count += (edges[e + 1].val - edges[e].val);
            //LogUtil.LogLine($"{edges[e].val}->{edges[e + 1].val}  ==> +{(edges[e + 1].val - edges[e].val)} => {ingredient_count}");
        }
        else if (depth == 0)
        {
            ingredient_count += 1;
            //LogUtil.LogLine($"End {edges[e].val}  ==> +1 => {ingredient_count}");

        }
    }
    ingredient_count++;
    LogUtil.LogLine($"{ingredient_count}");
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