using AOC_Util; 
using System.Data;
using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;
using System.Linq;
using System.Numerics;
using System.Reflection;
using System.Text.RegularExpressions;

using DataSet = AOC_Util.DataFull;
//using DataSet = AOC_Util.DataTest;

static char FlipSwitches(char old, char flips)
{
    return (char)((int)old ^ (int)flips);
}

void Part1(string filename)
{
    var lines = File.ReadAllLines(filename);
    var computers = lines.Select(l => Computer.Load(l)).ToList();

    var totalPressCount = 0;
    foreach (var c in computers)
    {
        var minPressCount = 0;
        var work = new Queue<(int depth, char val)>([ (0, '\0') ]);
        while (minPressCount == 0)
        {
            (var depth, var val) = work.Dequeue();
            foreach (var bf in c.buttonFlips)
            {
                var newVal = FlipSwitches(val, bf);
                if (newVal == c.switchTarget)
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

//void Part2_wrong(string filename)
//{
//    var lines = File.ReadAllLines(filename);
//    var computers = lines.Select(l => Computer.Load(l)).ToList();

//    var totalPressCount = 0;
//    foreach (var c in computers)
//    {
//        var minPressCount = 0;
//        var work = new Queue<(int depth, Int128 val)>([(0, '\0')]);
//        while (minPressCount == 0)
//        {
//            (var depth, var val) = work.Dequeue();
//            foreach (var inc in c.buttonJoltageIncrementsPacked)
//            {
//                // For each 
//                var newVal = val + inc;
//                if (newVal == c.joltageTargetPacked)
//                {
//                    minPressCount = depth + 1;
//                    break;
//                }
//                bool stillPossible = true;
//                for (int b = 0; b < 16 && stillPossible; b++)
//                {
//                    Int128 mask = (Int128)0xff << (8 * b);
//                    stillPossible = stillPossible && ((c.joltageTargetPacked & mask) >= (inc & mask));
//                }
//                if (stillPossible && work.Where((depth, val) => val == newVal).Count() == 0)
//                {
//                    work.Enqueue((depth + 1, newVal));
//                }
//            }
//        }
//        totalPressCount += minPressCount;
//    }
//    LogUtil.LogLine($"{totalPressCount}");
//}

string disp(int[] target)
{
    return target.Aggregate("", (a, n) => a + " " + n.ToString());
}
bool isValid(int[] target, int[] steps)
{
    Debug.Assert(target.Length == steps.Length);
    return Enumerable.Range(0, target.Length).All(i => target[i] - steps[i] >= 0);
}
int dot(int[] target, int[] steps)
{
    Debug.Assert(target.Length == steps.Length);
    int value = 0;
    for (int i = 0; i < target.Length; i++)
    {
        value += target[i] * steps[i];
    }
    return value;
}
int[] sub(int[] a, int[] b)
{
    Debug.Assert(a.Length == b.Length);
    int[] ret = (int[])a.Clone();
    for (int i = 0; i < ret.Length; i++)
    {
        ret[i] -= b[i];
    }
    return ret;
}
void accumulate(int[] acc, int[] inc)
{
    Debug.Assert(acc.Length == inc.Length);
    for (int i = 0; i < acc.Length; i++)
    {
        acc[i] += inc[i];
    }
}
int? FindMinStepsToWrong(int[] target, int[][] stepChoices, int currDepth)
{
    if (target.Any(d => d < 0))
    {
        return null;
    }
    if (target.All(d => d == 0))
    {
        return currDepth;
    }

    var newStepChoices = stepChoices.Where(s => isValid(target, s)).ToArray();
    newStepChoices.Sort((a, b) => dot(b, target).CompareTo(dot(a, target)));
    int[] accumulateMask = new int[target.Length];
    foreach (var step in newStepChoices)
    {
        accumulate(accumulateMask, step);
    }
    if (Enumerable.Range(0, target.Length).Any(i => accumulateMask[i] == 0 && target[i] > 0))
    {
        return null;
    }
    currDepth++;
    foreach (var step in newStepChoices)
    {
        var newTarget = sub(target, step);
        LogUtil.Log("".PadLeft(currDepth));
        LogUtil.LogLine($"{disp(target)} - {disp(step)} ({newStepChoices.Length})... {disp(newTarget)}");
        var res = FindMinStepsToWrong(newTarget, stepChoices, currDepth);
        if (res.HasValue)
        {
            return res;
        }
    }
    //LogUtil.LogLine($" => {disp(newTarget)}");
    return null;
}

BigInteger Factorial(int n)
{
    BigInteger result = 1;
    for (int i = 2; i <= n; i++)
        result *= i;
    return result;
}

void Part2(string filename)
{
    var lines = File.ReadAllLines(filename);
    var computers = lines.Select(l => Computer.Load(l)).ToList();

    var totalPressCount = 0;
    foreach ((var c, var i) in computers.Select((c, i) => (c, i)))
    {
        LogUtil.LogLine($"######{i}:");
        // Get the index of the bit which has the fewest number of buttons that can change it.  For a tie, go to the smaller Joltage
        var currBit = c.bitsToButtons.Select((b, index) => (b.Count, c.joltageTarget[index], index)).Min().index;
        // Stars and sticks = N=JoltageTarget, K=Number of buttons
        var N = c.joltageTarget[currBit];
        var K = c.bitsToButtons[currBit].Count;
        // N! / (K! * (N-K)!)
        var permutations = Factorial(N) / ((Factorial(K) * Factorial(N - K)));
        LogUtil.LogLine($"{i}====> {permutations}");
    }
    LogUtil.LogLine($"{totalPressCount}");
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
    public char switchTarget;
    public required char[] buttonFlips;
    public required int[][] buttonsToBits;
    public required List<int>[] bitsToButtons;
    public required int[] joltageTarget;

    public static Computer Load(string line)
    {
        //[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        //[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
        //[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        Regex regex = ComputerLineRegex();
        var match = regex.Match(line);
        Debug.Assert(match.Success && match.Groups.Count == 4);
        string switchTargetString = match.Groups[1].Value;
        string buttonStrings = match.Groups[2].Value;
        string joltageTargetString = match.Groups[3].Value;

        char switchTarget = switchTargetString.ToCharArray().Reverse().Aggregate('\0', (res, c) => (char)((int)res << 1 | (c == '#' ? 1 : 0)));
        List<char> buttonFlips = [];
        List<int[]> buttonsToBits = [];
        foreach (string buttonString in buttonStrings.Split().Select(s => s.Trim()))
        {
            Debug.Assert(buttonString[0] == '(' && buttonString[^1] == ')');
            char buttonFlip = buttonString[1..^1].Split(',').Aggregate('\0', (res, s) => (char)((int)res | (1 << int.Parse(s))));
            buttonFlips.Add(buttonFlip);
            Int128 buttonJoltageIncrementPacked = buttonString[1..^1].Split(',').Aggregate((Int128)0, (res, s) => res + ((Int128)1 << (8 * int.Parse(s))));
            int[] buttonJoltageIncrement = new int[switchTargetString.Length];
            foreach (string s in buttonString[1..^1].Split(','))
            {
                buttonJoltageIncrement[s[0] - '0'] = 1;
            }
            buttonsToBits.Add(buttonJoltageIncrement);
        }
        Int128 joltageTargetPacked = joltageTargetString.Split(',').Select(s => Int128.Parse(s)).Aggregate((res, i) => (res << 8) + i);
        int[] joltageTarget = [..joltageTargetString.Split(',').Select(s => int.Parse(s))];
        var bitsToButtons = new List<int>[joltageTarget.Length];
        for (int bitIndex = 0; bitIndex < joltageTarget.Length; bitIndex++)
        {
            bitsToButtons[bitIndex] = new List<int>();
            for (int buttonIndex = 0; buttonIndex < buttonsToBits.Count; buttonIndex++)
            {
                if (buttonsToBits[buttonIndex][bitIndex] > 0)
                {
                    bitsToButtons[bitIndex].Add(buttonIndex);
                }
            }
        }
        return new Computer
        {
            switchTarget = switchTarget,
            buttonFlips = [.. buttonFlips],
            buttonsToBits = [.. buttonsToBits],
            bitsToButtons = bitsToButtons,
            joltageTarget = joltageTarget
        };
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