namespace AOC.TwentyFour;
public class Day1 : DayBase
{
    public override string AnswerPart1(string input)
    {
        var left = new List<long>();
        var right = new List<long>();
        var locationLines = ReadLocationIdsFromLocationList(input)
            .Where(LineIsNotEmpty)
            .Select(l => l.Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries));
        foreach (var locations in locationLines)
        {
            left.Add(long.Parse(locations[0]));
            right.Add(long.Parse(locations[1]));
        }

        if (left.Count != right.Count)
        {
            throw new InvalidOperationException("Cannot operate on uneven lists.");
        }

        left.Sort();
        right.Sort();

        return left.Zip(right)
            .Sum(locations => Math.Abs(locations.First - locations.Second))
            .ToString();
    }

    public override string AnswerPart2(string input)
    {
        var similarities = new Dictionary<string, int>();
        var runningTotal = new List<Func<long>>();
        var locationLines = ReadLocationIdsFromLocationList(input)
            .Where(LineIsNotEmpty)
            .Select(l => l.Split(' ', StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries));
        foreach (var locations in locationLines)
        {
            var location = locations[0];
            var otherLocation = locations[1];
            runningTotal.Add(() => {
                int numTimes;
                if (similarities.TryGetValue(location, out numTimes) == false)
                {
                    numTimes = 0;
                }
                Console.WriteLine($"long.Parse({location}) * {numTimes} = {long.Parse(location) * numTimes}");
                return long.Parse(location) * numTimes;
            });
            if (similarities.TryAdd(otherLocation, 1) == false)
            {
                similarities[otherLocation] += 1;
            }
        }
        return runningTotal.Sum(rt => rt()).ToString();
    }

    private static IEnumerable<string> ReadLocationIdsFromLocationList(string input) => ConvertInputToLines(input);
}
