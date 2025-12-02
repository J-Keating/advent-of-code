using System.Data;
using System.Diagnostics;

//using DataSet = DataFull;
using DataSet = DataTest;

Stopwatch watch = new Stopwatch();
watch.Start();
var input = File.ReadAllLines(DataSet.Filename);
Run(input);
watch.Stop();
Console.WriteLine($"Completed in {watch.ElapsedMilliseconds}ms");

void Run(string[] data)
{
    Console.WriteLine($"Hello, {Config.Name} : {String.Join(", ", data)}!");
}

static class Config
{
    public static readonly string Name = "Dx";
}
static class DataFull
{
    public static readonly string Filename = "input.txt";
}
static class  DataTest
{
    public static readonly string Filename = "input_test.txt";
}