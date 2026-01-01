using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Reflection;

using AOC_Util;
//using DataSet = AOC_Util.DataFull;
using DataSet = AOC_Util.DataTest;

using PointRC = (System.Int64 row, System.Int64 col);
//using Rect = (PointRC p1, PointRC p2);

PointRC[] LoadFile(string filename)
{
    var lines = File.ReadAllLines(filename);
    PointRC[] locs = lines.Select(l =>
    {
        var split = l.Split(',');
        Debug.Assert(split.Length == 2);
        return (Int64.Parse(split[1]), Int64.Parse(split[0]));
    }).ToArray();
    return locs;
}

void Part1(string filename)
{
    PointRC[] locs = LoadFile(filename);
    locs.Sort();
    Int64 maxArea = 0;
    for (int i = 0; i < locs.Length; i++)
    {
        for (int j = i + 1; j < locs.Length; j++)
        {
            var p1 = locs[i];
            var p2 = locs[j];
            Debug.Assert(p2.row - p1.row >= 0);
            var area = (p2.row - p1.row + 1) * (Math.Abs(p1.col - p2.col) + 1);
            maxArea = Math.Max(maxArea, area);
        }
    }
    LogUtil.LogLine($"{maxArea}");
}

bool isInRect(PointRC p1, PointRC p2, PointRC pTest)
{
    return (p1.row < pTest.row && pTest.row < p2.row &&
            p1.col < pTest.col && pTest.col < p2.col);
}

bool SegmentPermits(Segment segment, PointRC p1, PointRC p2)
{
    bool ret = true;
    switch (segment.walltype)
    {
        case WallType.North:
            break;
        case WallType.South:
            break;
        case WallType.East:
            break;
        case WallType.West:
            break;
        default:
            throw new InvalidDataException();
    }
    return ret;
}

void Part2(string filename)
{
    PointRC[] locs = LoadFile(filename);
    locs.Sort();
    Int64 maxArea = 0;
    for (int i = 0; i < locs.Length; i++)
    {
        for (int j = i + 1; j < locs.Length; j++)
        {
            var p1 = locs[i];
            var p2 = locs[j];
            Debug.Assert(p2.row - p1.row >= 0);
            var area = ((Math.Abs(p1.row - p2.row) + 1) * (Math.Abs(p1.col - p2.col) + 1));
            if (area > maxArea)
            {
                bool isValid = !locs[i..j].Any(p => isInRect(p1, p2, p));
                if (isValid)
                {
                    maxArea = Math.Max(maxArea, area);
                }
            }
        }
    }
    LogUtil.LogLine($"{maxArea}");
}

void Run()
{
    LogUtil.Log($"{Config.Name}: Part1: ");
    LogUtil.Time(() => Part1(DataSet.Filename));
    LogUtil.Log($"{Config.Name}: Part2: ");
    LogUtil.Time(() => Part2(DataSet.Filename));
}

Run();

enum WallType
{
    North,
    South,
    East,
    West
};

class Segment
{
    public WallType walltype;
    public int location;
    public int min, max;
}

public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
}

//D9: Part1: 4781377701
//completed in 6ms
//D9: Part2: input.txt
//completed in 0ms