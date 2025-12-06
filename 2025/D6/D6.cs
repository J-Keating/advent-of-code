using AOC;
using System;
using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using DataSet = AOC.DataFull;
//using DataSet = AOC.DataTest;

void Part1(string filename)
{
    var lines = File.ReadAllLines(filename);
    var rows = lines[0..^1].Select(l => l.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(n => Int64.Parse(n)).ToArray()).ToArray();
    var ops = lines[^1].Split(' ', StringSplitOptions.RemoveEmptyEntries).ToArray();
    foreach (var row in rows)
    {
        Debug.Assert(row.Length == ops.Length);
    }
    // Readable
    Int64 final = 0;
    for (int i = 0; i < ops.Length; i++)
    {
        Int64 current = 0;
        switch (ops[i])
        {
            case "+":
                current = 0;
                for (int j = 0; j < rows.Length; j++)
                {
                    current += rows[j][i];
                }
                break;
            case "*":
                current = 1;
                for (int j = 0; j < rows.Length; j++)
                {
                    current *= rows[j][i];
                }
                break;
            default:
                throw new InvalidOperationException($"Unknown operator: {ops[i]}");

        }
        final += current;
    }
    // Not readable, but fewer lines, so it must be better, right?  Right????
    Int64 final2 = ops.Select((op, col) => (op, col)).Aggregate(0L,(acc, d) => acc + d.op switch
        {
            "+" => GridUtil.ColumnData_AA(rows, d.col, (0..rows.Length)).Aggregate((acc, x) => acc + x),
            "*" => GridUtil.ColumnData_AA(rows, d.col, (0..rows.Length)).Aggregate(1L, (acc, x) => acc * x),
            _ => throw new InvalidDataException()
        });
    Debug.Assert(final == final2);
    LogUtil.LogLine($"{final}");
}

void Part2(string filename)
{
    var data = FileUtil.LoadAsCharArray(filename);
    int opRow = data.GetLength(0) - 1;
    Int64 final = 0;
    int currCol = 0;
    while (currCol < data.GetLength(1))
    {
        char op = data[opRow, currCol];
        Int64 currOpResult = op switch
        {
            '+' => 0,
            '*' => 1,
            _ => throw new InvalidDataException()
        };
        while (currCol < data.GetLength(1))
        {
            Int64 currNum = GridUtil.ColumnData2d(data, currCol, (0..opRow)).Where(c => c != ' ').Aggregate(0L, (res, c) => res * 10 + (c - '0'));
            currCol++;
            if (currNum == 0)
            {
                break;
            }
            //LogUtil.Log($"{currNum} {op}");
            currOpResult = op switch
            {
                '+' => currOpResult + currNum,
                '*' => currOpResult * currNum,
                _ => throw new InvalidDataException()
            };
        }
        //LogUtil.LogLine($" = {currOpResult}");
        final += currOpResult;
    }
    LogUtil.LogLine($"{final}");
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

//D6: Part1: 5667835681547
//completed in 5ms
//D6: Part2: 9434900032651
//completed in 2ms