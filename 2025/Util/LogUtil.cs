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
    public static class LogUtil
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

    public static class  FileUtil
    {
        public static char[,] LoadAsCharArray(string filename)
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
    }

    public static class GridUtil
    {
        public static IEnumerable<(int, int)> GetValidNeighborsPos((int, int)[] deltas, char[,] grid, int r, int c)
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

        public static IEnumerable<(int, int)> Get4NeighborsPos(char[,] grid, int r, int c)
        {
            var deltas = new (int dx, int dy)[]
            {
        (-1, 0), (1, 0), (0, -1), (0, 1)
            };
            return GetValidNeighborsPos(deltas, grid, r, c);
        }

        public static IEnumerable<char> Get4NeighborsValue(char[,] grid, int r, int c)
        {
            return Get4NeighborsPos(grid, r, c).Select(pos => grid[pos.Item1, pos.Item2]);
        }

        public static IEnumerable<(int, int)> Get8NeighborsPos(char[,] grid, int r, int c)
        {
            var deltas = new (int dx, int dy)[]
            {
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1),           (0, 1),
                (1, -1),  (1, 0),  (1, 1)
            };
            return GetValidNeighborsPos(deltas, grid, r, c);
        }

        public static IEnumerable<char> Get8NeighborsValue(char[,] grid, int r, int c)
        {
            return Get8NeighborsPos(grid, r, c).Select(pos => grid[pos.Item1, pos.Item2]);
        }

        public static IEnumerable<T> ColumnData_AA<T>(T[][] data, int col, Range? rowRangeIn = null)
        {
            Range rowRange = rowRangeIn ?? (0..data.Length);
            foreach (int row in Enumerable.Range(rowRange.Start.Value, rowRange.End.Value - rowRange.Start.Value))
            {
                yield return data[row][col];
            }
        }

        public static IEnumerable<T> ColumnData2d<T>(T[,] data, int col, Range? rowRangeIn = null)
        {
            Range rowRange = rowRangeIn ?? (0..data.GetLength(0));
            foreach (int row in Enumerable.Range(rowRange.Start.Value, rowRange.End.Value - rowRange.Start.Value))
            {
                yield return data[row, col];
            }
        }

        public static IEnumerable<T> RowData2d<T>(T[,] data, int row, Range? colRangeIn = null)
        {
            Range colRange = colRangeIn ?? (0..data.GetLength(1));
            foreach (int col in Enumerable.Range(colRange.Start.Value, colRange.End.Value - colRange.Start.Value))
            {
                yield return data[row, col];
            }
        }
    }
}
