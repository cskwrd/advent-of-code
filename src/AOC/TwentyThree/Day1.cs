
namespace AOC.TwentyThree;
public class Day1 : DayBase
{
    public override string AnswerPart1(string input) => ReadCalibrationValuesFromCalibrationDocument(input)
        .Where(LineIsNotEmpty)
        .Select(SelectFirstLastDigitStrategyEZ)
        .Select(long.Parse)
        .Sum()
        .ToString();

    public override string AnswerPart2(string input) => ReadCalibrationValuesFromCalibrationDocument(input)
        .Where(LineIsNotEmpty)
        .Select(SelectFirstLastDigitStrategy)
        .Select(long.Parse)
        .Sum()
        .ToString();

    private static IEnumerable<string> ReadCalibrationValuesFromCalibrationDocument(string input) => ConvertInputToCalibrationData(input);

    private static string SelectFirstLastDigitStrategyEZ(string d, int index)
    {
        try
        {
            return $"{d.First(char.IsDigit)}{d.Last(char.IsDigit)}";
        }
        catch
        {
            throw new Exception($"Bad word at: '{index}'");
        }
    }

    private static readonly Dictionary<string, string> _digits = new()
    {
        ["ONE"] = "1",
        ["TWO"] = "2",
        ["THREE"] = "3",
        ["FOUR"] = "4",
        ["FIVE"] = "5",
        ["SIX"] = "6",
        ["SEVEN"] = "7",
        ["EIGHT"] = "8",
        ["NINE"] = "9",
        ["0"] = "0",
        ["1"] = "1",
        ["2"] = "2",
        ["3"] = "3",
        ["4"] = "4",
        ["5"] = "5",
        ["6"] = "6",
        ["7"] = "7",
        ["8"] = "8",
        ["9"] = "9",
    };
    public static string SelectFirstLastDigitStrategy(string d)
    {
        d = d.ToUpperInvariant();
        KeyValuePair<int, string> left = new(d.Length, string.Empty);
        KeyValuePair<int, string> right = new(-1, string.Empty);
        int index;
        foreach (var digit in _digits)
        {
            index = d.IndexOf(digit.Key);
            if (index >= 0 && index <= left.Key) // <= feels a little wrong because there shouldnt be any overlapping indices, but just < feels more wrong
            {
                left = new(index, digit.Value);
            }
            index = d.LastIndexOf(digit.Key);
            if (index >= right.Key) // >= feels a little wrong because there shouldnt be any overlapping indices, but just > feels more wrong
            {
                right = new(index, digit.Value);
            }
        }
        return $"{left.Value}{right.Value}";
    }

    private static IEnumerable<string> ConvertInputToCalibrationData(string input)
    {
        if (string.IsNullOrWhiteSpace(input) == false)
        {
            return input.Split(Environment.NewLine, StringSplitOptions.TrimEntries);
        }
        return Enumerable.Empty<string>();
    }
}
