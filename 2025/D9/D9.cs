using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Reflection;

using AOC_Util;
//using DataSet = AOC_Util.DataFull;
using DataSet = AOC_Util.DataTest;

using PointRC = (System.Int64 row, System.Int64 col);
using Segment = (System.Int64 start, System.Int64 end);
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
            ref readonly var p1 = ref locs[i];
            ref readonly var p2 = ref locs[j];
            Debug.Assert(p2.row - p1.row >= 0);
            var area = (p2.row - p1.row + 1) * (Math.Abs(p1.col - p2.col) + 1);
            maxArea = Math.Max(maxArea, area);
        }
    }
    LogUtil.LogLine($"{maxArea}");
}

bool Between(Int64 test, Int64 start, Int64 end)
{
    return (start <= test && test <= end) || (end <= test && test <= start);
}

bool IsInside(List<Wall> wallsHorizontal, List<Wall> wallsVertical, Int64 col)
{
    if (wallsHorizontal.Any(w => w.start.col <= col && col <= w.stop.col))
    {
        return true;
    }
    int before = -1;
    int after = 0;
    while (after < wallsVertical.Count && col < wallsVertical[after].start.col)
    {
        before = after;
        after++;
    }
    if (before == -1 || after == wallsVertical.Count)
    {
        return false;
    }

}

void Part2(string filename)
{
    PointRC[] locs = LoadFile(filename);
    List<Wall> allWalls = new();
    for (var i = 0; i < locs.Length; i++)
    {
        ref readonly var p1 = ref locs[i];
        ref readonly var p2 = ref locs[(i + 1) % locs.Length];
        allWalls.Add(Wall.Create(in p1, in p2));
    }
    locs.Sort();
    Int64 maxArea = 0;
    for (int i = 0; i < locs.Length; i++)
    {
        for (int j = i + 1; j < locs.Length; j++)
        {
            ref readonly var p1 = ref locs[i];
            ref readonly var p2 = ref locs[j];
            Debug.Assert(p2.row - p1.row >= 0);
            var area = (p2.row - p1.row + 1) * (Math.Abs(p1.col - p2.col) + 1);
            if (area > maxArea)
            {
                bool valid = true;
                var testSegment = new Segment { start = p1.col, end = p2.col };
                if (testSegment.start > testSegment.end)
                {
                    Util.Swap(ref testSegment.start, ref testSegment.end);
                }
                Int64 currRow = p1.row;
                while (valid && currRow <= p2.row)
                {
                    var wallsHorizontal = new List<Wall>();
                    var wallsVertical = new List<Wall>();
                    // Find relevant walls
                    foreach (var wall in allWalls)
                    {
                        switch (wall.wallType)
                        {
                            case WallType.Up or WallType.Down:
                                if (Between(currRow, wall.start.row, wall.stop.row))
                                {
                                    wallsHorizontal.Add(wall);
                                }
                                break;
                            case WallType.Right or WallType.Left:
                                if (wall.start.row == currRow)
                                {
                                    wallsVertical.Add(wall);
                                }
                                break;
                            default:
                                throw new InvalidDataException();
                        }
                    }
                    //walls.Sort((w1, w2) =>
                    //{
                    //    var cmp = w1.start.col.CompareTo(w2.start.col);
                    //    if (cmp != 0) return cmp;
                    //    Debug.Assert(w1.wallType != w2.wallType);
                    //    return w1.wallType.CompareTo(w2.wallType); // W, N, S, E
                    //});
                    //Debug.Assert(wallsVertical.Length == 0 || wallsVertical[0].wallType == WallType.West);
                    // Find segments defined by walls
                    currRow++;
                }

                if (valid)
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
    // Order is important
    Up,
    Right,
    Left,
    Down,
};

class Wall
{
    public WallType wallType;
    public PointRC start;
    public PointRC stop;
    //public int location;
    //public int min, max;

    public static Wall Create(ref readonly PointRC a, ref readonly PointRC b)
    {
        Debug.Assert(a.row == b.row || a.col == b.col);
        WallType wallType = (b.row - a.row) switch
        {
            > 0 => WallType.Down,
            < 0 => WallType.Up,
            0 => (b.col - a.col) switch
            {
                > 0 => WallType.Right,
                < 0 => WallType.Left,
                0 => throw new InvalidDataException()
            }
        };
        var ret = new Wall { wallType = wallType, start = a, stop = b};
        if (wallType == WallType.Left || wallType == WallType.Up)
        {
            Util.Swap(ref ret.start, ref ret.stop);
        }
        return ret;
    }
}

public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
}

//D9: Part1: 4781377701
//completed in 6ms
//D9: Part2: input.txt
//completed in 0ms