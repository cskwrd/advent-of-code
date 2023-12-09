namespace AOC;

public abstract class DayBase
{
    protected static IEnumerable<string> ConvertInputToLines(string input)
    {
        if (string.IsNullOrWhiteSpace(input) == false)
        {
            var lines = input.Split(Environment.NewLine, StringSplitOptions.TrimEntries);
            if (string.IsNullOrWhiteSpace(lines[^1]))
            {
                return lines;
            }
            return lines.Concat([string.Empty]); // ensure there is always an empty line at the end
        }
        return Enumerable.Empty<string>();
    }

    /// <summary>
    /// Converts <see cref="Enumerable"/> representing lines in a file into blocks indicated by empty lines.
    /// </summary>
    /// <returns>Blocks of lines.</returns>
    /// <remarks>Relies on very last line of input being empty.</remarks>
    protected static Func<string, IEnumerable<ICollection<string>>> BlocksFromInputLines()
    {
        List<string> chunk = [];
        return MemoizedFunc;
        IEnumerable<ICollection<string>> MemoizedFunc(string line)
        {
            if (string.IsNullOrWhiteSpace(line))
            {
                yield return chunk;
                chunk = [];
            }
            else
            {
                chunk.Add(line);
            }
        }
    }

    protected static bool LineIsNotEmpty(string line) => string.IsNullOrWhiteSpace(line) is not true;
    public abstract string AnswerPart1(string input);
    public abstract string AnswerPart2(string input);
}
