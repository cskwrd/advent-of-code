
namespace AOC.TwentyTwo;
public class Day1 : DayBase
{
    public override string AnswerPart1(string input) => TakeInventory(input).Max().ToString();
    public override string AnswerPart2(string input) => TakeInventory(input).OrderDescending().Take(3).DefaultIfEmpty(0).Sum().ToString();

    private static IEnumerable<long> TakeInventory(string input)
    {
        var pocketContents = ConvertInputToPocketContents(input);

        return pocketContents.Select(c => c.Select(long.Parse).Sum()).DefaultIfEmpty(0);
    }

    private static IEnumerable<ICollection<string>> ConvertInputToPocketContents(string input)
    {
        if (string.IsNullOrWhiteSpace(input) == false)
        {
            var contents = input.Split(Environment.NewLine, StringSplitOptions.TrimEntries);
            var pocketContents = new List<string>();

            foreach (var item in contents)
            {
                if (string.IsNullOrWhiteSpace(item))
                {
                    yield return pocketContents;
                    pocketContents.Clear();
                    continue;
                }
                pocketContents.Add(item);
            }

            if (pocketContents.Any())
            {
                yield return pocketContents;
            }
        }
    }
}
