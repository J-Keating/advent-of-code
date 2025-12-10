using System.Data;
using System.Diagnostics;
using System.Reflection;
using System.Text.RegularExpressions;

using AOC_Util;

using DataSet = AOC_Util.DataFull;
//using DataSet = AOC_Util.DataTest;

static char Press(char old, char flips)
{
    return (char)((int)old ^ (int)flips);
}

void Part1(string filename)
{
    var lines = File.ReadAllLines(filename);
    var computers = lines.Select(l => Computer.Load(l)).ToList();
    LogUtil.LogLine($"{lines[0]}");
    var b = computers[0].buttonFlips;
    Debug.Assert(computers[0].target == Press(Press(Press('\0', b[0]), b[1]), b[2]));

    var totalPressCount = 0;
    foreach (var c in computers)
    {
        var minPressCount = 0;
        var work = new Queue<(int depth, char val)>( [(0, '\0')]);
        while (minPressCount == 0)
        {
            (var depth, var val) = work.Dequeue();
            foreach (var bf in c.buttonFlips)
            {
                var newVal = Press(val, bf);
                if (newVal == c.target)
                {
                    minPressCount = depth + 1;
                    break;
                }
                work.Enqueue((depth + 1, newVal));
            }
        }
        totalPressCount += minPressCount;
    }
    LogUtil.LogLine($"{totalPressCount}");
}

void Part2(string filename)
{
}

void Run()
{
    LogUtil.Log($"{Config.Name}: Part1: ");
    LogUtil.Time(() => Part1(DataSet.Filename));
    LogUtil.Log($"{Config.Name}: Part2: ");
    LogUtil.Time(() => Part2(DataSet.Filename));
}

Run();

partial class Computer
{
    public char target;
    public char[] buttonFlips;

    public static Computer Load(string line)
    {
        //[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        //[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        //[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        Regex regex = ComputerLineRegex();
        var match = regex.Match(line);
        Debug.Assert(match.Success && match.Groups.Count == 4);
        string targetString = match.Groups[1].Value;
        string buttonString = match.Groups[2].Value;
        string joltageString = match.Groups[3].Value;

        char target = targetString.ToCharArray().Reverse().Aggregate('\0', (res, c) => (char)((int)res << 1 | (c == '#' ? 1 : 0)));
        List<char> buttons = new();
        foreach (string oneButtonString in buttonString.Split().Select(s => s.Trim()))
        {
            Debug.Assert(oneButtonString[0] == '(' && oneButtonString[^1] == ')');
            char oneButton = oneButtonString[1..^1].Split(',').Aggregate('\0', (res, s) => (char)((int)res | (1 << s[0] - '0')));
            buttons.Add(oneButton);
        }
        return new Computer { target = target, buttonFlips = buttons.ToArray() };
    }

    [GeneratedRegex("""\[([\.#]+)\] (.+) \{([\d,]+)\}""")]
    private static partial Regex ComputerLineRegex();
}

public static class Config
{
    public static readonly string? Name = Assembly.GetExecutingAssembly()?.GetName()?.Name;
}

//D7: Part1: 1678
//completed in 6ms
//D7: Part2: 357525737893560
//completed in 6ms