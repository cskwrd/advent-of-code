

namespace AOC.TwentyThree;
public class Day2 : DayBase
{
    private const int _red = 12;
    private const int _green = 13;
    private const int _blue = 14;
    public override string AnswerPart1(string input)
    {
        var lines = ConvertInputToLines(input);
        var nonEmptyLines = lines.Where(LineIsNotEmpty);
        var games = nonEmptyLines.Select(GameFromLine);
        var validGames = games.Where(GamePossibleBasedOnParameters);
        var answer = validGames.Select(g => g.Id).Sum().ToString();
        return answer;
    }

    public override string AnswerPart2(string input)
    {
        var lines = ConvertInputToLines(input);
        var nonEmptyLines = lines.Where(LineIsNotEmpty);
        var games = nonEmptyLines.Select(GameFromLine);
        var minSetsRequired = games.Select(MinRequiredCubeSetFromGame);
        var answer = minSetsRequired.Select(s => s.Red * s.Green * s.Blue).Sum().ToString();
        return answer;
    }

    private Set MinRequiredCubeSetFromGame(Game game)
    {
        long red = 0, green = 0, blue = 0;
        foreach (var set in game.Sets)
        {
            red = Math.Max(red, set.Red);
            green = Math.Max(green, set.Green);
            blue = Math.Max(blue, set.Blue);
        }
        return new Set(red, green, blue);
    }

    private bool GamePossibleBasedOnParameters(Game game) => game.Sets.All(s => s.Red <= _red && s.Green <= _green && s.Blue <= _blue);

    private Game GameFromLine(string line)
    {
        var gameData = line.Split(":", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
        var gameId = int.Parse(new(gameData[0].SkipWhile(c => char.IsDigit(c) == false).ToArray()));
        var gameSets = gameData[1].Split(";", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
            .SelectMany(SetFromGame)
            .ToList();
        return new()
        {
            Id = gameId,
            Sets = gameSets,
        };
    }

    private IEnumerable<Set> SetFromGame(string set)
    {
        if (string.IsNullOrWhiteSpace(set) == false)
        {
            var setData = set.Split(",", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries)
                .Select(ColorsFromSet);
            int red = 0, green = 0, blue = 0;
            foreach (var colorData in setData)
            {
                switch (colorData.color)
                {
                    case "red":
                        red = colorData.numCubes;
                        break;
                    case "green":
                        green = colorData.numCubes;
                        break;
                    case "blue":
                        blue = colorData.numCubes;
                        break;
                    default:
                        throw new Exception($"Unknown color: '{colorData.color}'");
                }
            }
            yield return new Set(red, green, blue);
        }
    }

    private (string color, int numCubes) ColorsFromSet(string cubeData)
    {
        var colorData = cubeData.Split(" ", StringSplitOptions.RemoveEmptyEntries | StringSplitOptions.TrimEntries);
        return (color: colorData[1], numCubes: int.Parse(colorData[0]));
    }

    private class Game
    {
        public int Id { get; set; }
        public required List<Set> Sets { get; set; }
    }

    private record Set(long Red, long Green, long Blue);
}
