using System.Data;
using System.Diagnostics;
using System.Numerics;
using System.Reflection;
using System.Runtime.CompilerServices;

using AOC_Util;

using DataSet = AOC_Util.DataFull;
//using DataSet = AOC_Util.DataTest;

void Part1(string filename)
{
    int foundCount = 0;
    var grid = FileUtil.LoadAsCharArray(filename);
    for (int r = 0; r < grid.GetLength(0); r++)
    {
        for (int c = 0; c < grid.GetLength(1); c++)
        {
            if (grid[r, c] == '@' && GridUtil.Get8NeighborsValue(grid, r, c).Count(v => v == '@') < 4)
            {
                //LogUtil.LogLine($"Found at ({r},{c})");
                foundCount++;
            }
        }
    }
    LogUtil.LogLine($"Total: {foundCount}");
}

void Part2(string filename)
{
    int foundCount = 0;
    HashSet<(int, int)> changeableRolls = new();
    var grid = FileUtil.LoadAsCharArray(filename);
    while (true)
    {
        for (int r = 0; r < grid.GetLength(0); r++)
        {
            for (int c = 0; c < grid.GetLength(1); c++)
            {
                if (grid[r, c] == '@' && GridUtil.Get8NeighborsValue(grid, r, c).Count(v => v == '@') < 4)
                {
                    //LogUtil.LogLine($"Found at ({r},{c})");
                    changeableRolls.Add((r, c));
                    foundCount++;
                }
            }
        }
        foreach ((var r, var c) in changeableRolls)
        {
            grid[r, c] = '.';
        }
        if (changeableRolls.Count == 0)
        {
            break;
        }
        changeableRolls.Clear();
    }
    LogUtil.LogLine($"Total: {foundCount}");
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

//D4: Part1: Total: 1320
//completed in 10ms
//D4: Part2: Total: 8354
//completed in 138ms