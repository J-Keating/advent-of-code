using System.Diagnostics;

namespace AOC
{
    public static class DataFull
    {
        public static readonly string Filename = "input.txt";
    }
    public static class DataTest
    {
        public static readonly string Filename = "input_test.txt";
    }
    public static class Util
    {
        public static void Log(string message)
        {
            Console.Write(message);
            Debug.Write(message);
        }
        public static void LogLine(string message)
        {
            Console.WriteLine(message);
            Debug.WriteLine(message);
        }

        public static void Time(Action action)
        {
            Stopwatch sw = new();
            sw.Start();
            action();
            sw.Stop();
            LogLine($"completed in {sw.ElapsedMilliseconds}ms");
        }
    }
}
