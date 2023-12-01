
namespace AOC.TwentyThree;
public class Day1 : DayBase
{
    public override string AnswerPart1(string input) => DecipherCalibrationDocument(input, FilterNonDigitsStrategy).Sum().ToString();
    public override string AnswerPart2(string input) => ReadCalibrationValuesFromCalibrationDocument(input)
        .Where(v => string.IsNullOrWhiteSpace(v) == false)
        .Select(ConvertWordsToDigitsStrategy)
        .Select(FilterNonDigitsStrategy)
        .Select(SelectFirstLastDigitStrategy)
        .Select(long.Parse)
        .Sum()
        .ToString();

    private static IEnumerable<long> DecipherCalibrationDocument(string input, Func<string, string> ParseCalibrationValue)
    {
        var calibrationData = ConvertInputToCalibrationData(input);

        return calibrationData.Select(ParseCalibrationValue)
            .Where(d => !string.IsNullOrWhiteSpace(d))
            .Select(d => $"{d.First()}{d.Last()}")
            .Select(long.Parse)
            .DefaultIfEmpty(0);
    }

    private static IEnumerable<string> ReadCalibrationValuesFromCalibrationDocument(string input) => ConvertInputToCalibrationData(input);

    private static string FilterNonDigitsStrategy(string d) => new(d.Where(char.IsDigit).ToArray());

    private static string SelectFirstLastDigitStrategy(string d, int index)
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
    
    private static readonly Dictionary<string, string> _replacibles = new()
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
    };
    public static string ConvertWordsToDigitsStrategy(string d)
    {
        d = d.ToUpperInvariant();
        SortedDictionary<int, KeyValuePair<string, string>> replaceActions;
        int index;
        do
        {
            replaceActions = new();
            foreach (var replacible in _replacibles)
            {
                index = d.IndexOf(replacible.Key);
                if (index >= 0)
                {
                    replaceActions.Add(index, replacible);
                }
            }
            foreach (var replaceAction in replaceActions)
            {
                var action = replaceAction.Value;
                d = d.Replace(action.Key, action.Value);
            }
        } while (replaceActions.Any());
        return d;
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
