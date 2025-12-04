using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;

using AOC;

using DataSet = AOC.DataFull;
//using DataSet = AOC.DataTest;

char[,] LoadAsCharArray(string filename)
{
    var lines = File.ReadAllLines(filename);
    int rowCount = lines.Length;
    int colCount = lines[0].Length;
    char[,] array = new char[rowCount, colCount];
    for (int r = 0; r < rowCount; r++)
    {
        for (int c = 0; c < colCount; c++)
        {
            array[r, c] = lines[r][c];
        }
    }
    return array;
}

IEnumerable<(int, int)> GetValidNeighborsPos((int,int)[] deltas, char[,] grid, int r, int c)
{
    int height = grid.GetLength(0);
    int width = grid.GetLength(1);
    foreach (var (dx, dy) in deltas)
    {
        int nx = r + dx;
        int ny = c + dy;
        if (nx >= 0 && nx < height && ny >= 0 && ny < width)
        {
            yield return (nx, ny);
        }
    }
}

IEnumerable<(int, int)> Get4NeighborsPos(char[,] grid, int r, int c)
{
    var deltas = new (int dx, int dy)[]
    {
        (-1, 0), (1, 0), (0, -1), (0, 1)
    };
    return GetValidNeighborsPos(deltas, grid, r, c);
}

IEnumerable<char> Get4NeighborsValue(char[,] grid, int r, int c)
{
    return Get4NeighborsPos(grid, r, c).Select(pos => grid[pos.Item1, pos.Item2]);
}

IEnumerable<(int, int)> Get8NeighborsPos(char[,] grid, int r, int c)
{
    var deltas = new (int dx, int dy)[]
    {
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    };
    return GetValidNeighborsPos(deltas, grid, r, c);
}

IEnumerable<char> Get8NeighborsValue(char[,] grid, int r, int c)
{
    return Get8NeighborsPos(grid, r, c).Select(pos => grid[pos.Item1, pos.Item2]);
}

void Part1(string filename)
{
    int foundCount = 0;
    var grid = LoadAsCharArray(filename);
    for (int r = 0; r < grid.GetLength(0); r++)
    {
        for (int c = 0; c < grid.GetLength(1); c++)
        {
            if (grid[r, c] == '@')
            {
                var n = Get8NeighborsValue(grid, r, c).ToArray();
                if (n.Count(v => v == '@') < 4)
                {
                    Util.LogLine($"Found at ({r},{c})");
                    foundCount++;
                }
                //Util.LogLine($"Pos: ({r},{c})  Value: {grid[r,c]}  Neighbors: {string.Join(",", Get4NeighborsValue(grid, r, c))}");
            }
        }
    }
    Util.LogLine($"Total: {foundCount}");
}

void Part2(string filename)
{
    var lines = File.ReadAllLines(filename);
    foreach (var line in lines)
    {
        //Util.LogLine($"Line: {line}  Voltage: {voltage_final}");
    }
    Util.LogLine($"Total: {filename}");
}

void Run()
{
    Util.Log($"{Config.Name}: Part1: ");
    Util.Time(() => Part1(DataSet.Filename));
    Util.Log($"{Config.Name}: Part2: ");
    Util.Time(() => Part2(DataSet.Filename));
}

Run();
public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
}

//D3: Part1: Total: 17405
//completed in 3ms
//D3: Part2: Total: 171990312704598
//completed in 66ms