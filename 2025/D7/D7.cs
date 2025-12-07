using AOC;
using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;
using DataSet = AOC.DataFull;
//using DataSet = AOC.DataTest;

void Part1(string filename)
{
    int totalSplits = 0;
    var grid = FileUtil.LoadAsCharArray(filename);
    var startLocs = GridUtil.RowData2d(grid, 0).Select((c, i) => (c, i)).Where(p => p.c == 'S').Select(p => p.i).ToArray();
    Debug.Assert(startLocs.Length == 1);
    var beamLocs = new HashSet<int>();
    beamLocs.Add(startLocs[0]);
    for (var row = 1; row < grid.GetLength(0); row++)
    {
        var newBeams = new HashSet<int>();
        foreach (var beam in beamLocs)
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
        beamLocs = newBeams;
    }
    LogUtil.LogLine($"{totalSplits}");
}

void Part2(string filename)
{
    int totalSplits = 0;
    LogUtil.LogLine($"{totalSplits}");
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
//completed in 19ms