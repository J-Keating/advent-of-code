using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;

using AOC;

//using DataSet = AOC.DataFull;
using DataSet = AOC.DataTest;

void Part1(string filename)
{
    LogUtil.LogLine($"{filename}");
}

void Part2(string filename)
{
    LogUtil.LogLine($"{filename}");
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
