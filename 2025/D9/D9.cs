using AOC_Util;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Reflection;
using static System.Net.Mime.MediaTypeNames;
using DataSet = AOC_Util.DataFull;
//using DataSet = AOC_Util.DataTest;
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
    int before = wallsVertical.FindLastIndex(w => w.start.col <= col);
    int after = wallsVertical.FindIndex(w => w.start.col >= col);
    //while (after < wallsVertical.Count && col < wallsVertical[after].start.col)
    //{
    //    before = after;
    //    after++;
    //}
    if (before == -1 || after == -1)
    {
        return false;
    }
    if (before == after)
    {
        return true;
    }
    //return wallsVertical[before].wallType == wallsVertical[after].wallType;
    int depth = 0;
    for (var i = 0; i < after; i++)
    {
        depth += wallsVertical[i].wallType switch {
            WallType.Up => 1,
            WallType.Down => -1,
            _ => throw new InvalidDataException()
        };
        depth = Math.Clamp(depth, -1, 1);
    }
    return depth != 0;
}

(List<Wall>, List<Wall>) GetWallsInRow(ref readonly List<Wall> allWalls, Int64 currRow)
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
                    wallsVertical.Add(wall);
                }
                break;
            case WallType.Right or WallType.Left:
                if (wall.start.row == currRow)
                {
                    wallsHorizontal.Add(wall);
                }
                break;
            default:
                throw new InvalidDataException();
        }
    }
    wallsHorizontal.Sort((w1, w2) =>
    {
        var cmp = w1.start.col.CompareTo(w2.start.col);
        Debug.Assert(cmp != 0);
        return cmp;
    });
    wallsVertical.Sort((w1, w2) =>
    {
        var cmp = w1.start.col.CompareTo(w2.start.col);
        if (cmp != 0) { return cmp; }
        Debug.Assert(w1.wallType != w2.wallType);
        return w1.wallType.CompareTo(w2.wallType); // U, D
    });
    return (wallsHorizontal, wallsVertical);
}

List<Segment> EnclosedSegments(ref readonly List<Wall> wallsHorizontal, ref readonly List<Wall> wallsVertical, Int64 currRow)
{
    var workingSegments = new List<Segment>();
    Int64? currSegmentStart = null;
    WallType? currSegmentEndType = null;
    foreach (var wall in wallsVertical)
    {
        if (currSegmentEndType.HasValue && currSegmentEndType == wall.wallType)
        {
            Debug.Assert(currSegmentStart.HasValue);
            workingSegments.Add(new Segment { start = currSegmentStart.Value, end = wall.start.col });
            currSegmentStart = null;
            currSegmentEndType = null;
        }
        else if (!currSegmentEndType.HasValue)
        {
            currSegmentStart = wall.start.col;
            currSegmentEndType = wall.wallType switch
            {
                WallType.Up => WallType.Down,
                WallType.Down => WallType.Up,
                _ => throw new InvalidDataException()
            };
        }
    }

    var ret = new List<Segment>();
    for (var i = 0; i < workingSegments.Count; i++)
    {
        // Combine adjacent segments which are connected by a horizontal wall
        if (i < workingSegments.Count - 1 &&  wallsHorizontal.Any(w => workingSegments[i].end == w.start.col && w.stop.col == workingSegments[i + 1].start))
        {
            var newSegment = new Segment { start = workingSegments[i].start, end = workingSegments[i + 1].end };
            ret.Add(newSegment);
            i++;
        }
        else
        {
            ret.Add(workingSegments[i]);
        }
    }
    return ret;
}

bool IsSegmentInsideOld(ref readonly List<Wall> allWalls, Int64 currRow, Segment testSegment)
{
    (var wallsHorizontal, var wallsVertical) = GetWallsInRow(in allWalls, currRow);
    bool valid = true;
    var col = testSegment.end;
    while (valid && col >= testSegment.start)
    {
        valid = valid && IsInside(wallsHorizontal, wallsVertical, col);
        col--;
    }
    return valid;
}

bool IsSegmentInside(ref readonly List<Wall> allWalls, Int64 currRow, Segment testSegment)
{
    (var wallsHorizontal, var wallsVertical) = GetWallsInRow(in allWalls, currRow);
    var segments = EnclosedSegments(ref wallsHorizontal, ref wallsVertical, currRow);
    return segments.Any(s => s.start <= testSegment.start && testSegment.end <= s.end);
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

    // Diagnostics
    for (Int64 row = 0; row <= 14; row++)
    {
        LogUtil.Log($"{row}: ");
        for (Int64 col = 0; col <= 14; col++)
        {
            var testSegment = new Segment { start = col, end = col };
            string test = IsSegmentInside(ref allWalls, row, testSegment) ? "X" : ".";
            LogUtil.Log($"{test}");
        }
        LogUtil.LogLine("");
    }
    for (Int64 row = 0; row < 14; row++)
    {
        (var wallsHorizontal, var wallsVertical) = GetWallsInRow(in allWalls, row);
        var s = EnclosedSegments(ref wallsHorizontal, ref wallsVertical, row);
        LogUtil.LogLine($"{row}: {s.Count}");
    }

    Int64 maxArea = 0;
    for (int i = 0; i < locs.Length; i++)
    {
        for (int j = locs.Length - 1; j > i; j--)
        {
            ref readonly var p1 = ref locs[i];
            ref readonly var p2 = ref locs[j];
            Debug.Assert(p2.row - p1.row >= 0);
            var area = (p2.row - p1.row + 1) * (Math.Abs(p1.col - p2.col) + 1);
            if (area > maxArea)
            {
                var testSegment = new Segment { start = p1.col, end = p2.col };
                if (testSegment.start > testSegment.end)
                {
                    Util.Swap(ref testSegment.start, ref testSegment.end);
                }
                // Insta-reject if any corners are within the test rectangle
                bool valid = true; // !locs[i..j].Any(l => testSegment.start < l.col && l.col < testSegment.end);
                Int64 currRow = p1.row;
                while (valid && currRow <= p2.row)
                {
                    valid = valid && IsSegmentInside(ref allWalls, currRow, testSegment);
                    currRow++;
                }

                if (valid)
                {
                    maxArea = Math.Max(maxArea, area);
                }
            }
        }
        LogUtil.LogLine($"{i}: {maxArea}");
    }
    LogUtil.LogLine($"{maxArea}");
}

void Run()
{
    LogUtil.Log($"{Config.Name}: Part1: ");
    LogUtil.Time(() => Part1(DataSet.Filename));
    LogUtil.Log($"{Config.Name}: Part2: ");
    LogUtil.Time(() => Part2(DataSet.Filename));
    //LogUtil.Time(() => Part2("input_test2.txt"));
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
//D9: Part2: 1470616992
// completed in 1827500ms