using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;

using AOC;

//using DataSet = AOC.DataFull;
using DataSet = AOC.DataTest;

void Part1(string filename)
{
    Util.LogLine($"{filename}");
}

void Part2(string filename)
{
    Util.LogLine($"{filename}");
}

void Run()
{
    Util.Log($"{Config.Name}: Part1: ");
    Util.Time(() => Part1(DataSet.Filename));
    Util.Log($"{Config.Name}: Part2: ");
    Util.Time(() => Part2(DataSet.Filename));
}

Run();