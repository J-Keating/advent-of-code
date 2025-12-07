using System.Data;
using System.Diagnostics;
using System.Reflection;

using AOC_Util;
using DataSet = AOC_Util.DataFull;
//using DataSet = AOC_Util.DataTest;

//using Beam = (int pos, int pathCount);
using BeamSet = System.Collections.Generic.Dictionary<int, System.Int64>;

void Part1(string filename)
{
    int totalSplits = 0;
    var grid = FileUtil.LoadAsCharArray(filename);
    var startLocs = GridUtil.RowData2d(grid, 0).Select((c, i) => (c, i)).Where(p => p.c == 'S').Select(p => p.i).ToArray();
    Debug.Assert(startLocs.Length == 1);
    var beams = new HashSet<int> { startLocs[0] };
    for (var row = 1; row < grid.GetLength(0); row++)
    {
        var newBeams = new HashSet<int>();
        foreach (var beam in beams)
        {
            switch(grid[row, beam])
            {
                case '.':
                    newBeams.Add(beam);
                    break;
                case '^':
                    totalSplits++;
                    if ((beam - 1) > 0) { newBeams.Add(beam - 1); }
                    if ((beam + 1) < grid.GetLength(1)) { newBeams.Add(beam + 1); }
                    break;
                default:
                    throw new InvalidDataException();
            }
        }
        beams = newBeams;
    }
    LogUtil.LogLine($"{totalSplits}");
}

void Part2(string filename)
{
    var grid = FileUtil.LoadAsCharArray(filename);
    var startLocs = GridUtil.RowData2d(grid, 0).Select((c, i) => (c, i)).Where(p => p.c == 'S').Select(p => p.i).ToArray();
    Debug.Assert(startLocs.Length == 1);
    var beams = new BeamSet { { startLocs[0], 1L } };
    for (var row = 1; row < grid.GetLength(0); row++)
    {
        var newBeams = new BeamSet();
        void InsertOrAddIfInRange(int pos, Int64 count)
        {
            if (0 <= pos && pos < grid.GetLength(1))
            {
                newBeams[pos] = newBeams.TryGetValue(pos, out var currCount) ? currCount + count : count;
            }
        }

        foreach ((int pos, Int64 count) in beams)
        {
            switch (grid[row, pos])
            {
                case '.':
                    InsertOrAddIfInRange(pos, count);
                    break;
                case '^':
                    InsertOrAddIfInRange(pos - 1, count);
                    InsertOrAddIfInRange(pos + 1, count);
                    break;
                default:
                    throw new InvalidDataException();
            }
        }
        beams = newBeams;
    }
    Int64 totalPaths = beams.Values.Aggregate((count, acc) => acc + count);
    LogUtil.LogLine($"{totalPaths}");
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