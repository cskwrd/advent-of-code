
using System.Linq;
using System.Runtime.CompilerServices;

namespace AOC.TwentyTwo;
public class Day3 : DayBase
{
    const int NUM_COMPARTMENTS = 2;
    public override string AnswerPart1(string input) => GetBackpacks(input).Select(bp => FindMispackedItem(bp.left, bp.right)).Select(ToItemPriority).Sum().ToString();

    public override string AnswerPart2(string input) => GetBackpacks(input).Chunk(3).Select(bps => FindMispackedItem(bps)).Select(ToItemPriority).Sum().ToString();

    private char FindMispackedItem((HashSet<char> left, HashSet<char> right)[] backpacks)
    {
        if (backpacks.Length < 2)
        {
            throw new ArgumentException("not enough backpacks");
        }
        var mispackedItems = backpacks[0].left.Concat(backpacks[0].right).Intersect(backpacks[1].left.Concat(backpacks[1].right));
        foreach (var backpack in backpacks)
        {
            mispackedItems = mispackedItems.Intersect(backpack.left.Concat(backpack.right));
        }
        return mispackedItems.Single();
    }

    private static int ToItemPriority(char item)
    {
        if (item < 'A' || item > 'z')
        {
            throw new ArgumentOutOfRangeException(nameof(item));
        }

        if (item >= 'a')
        {
            return item - 'a' + 1;
        }
        return item - 'A' + 27;
    }

    private static char FindMispackedItem(HashSet<char> leftCompartment, HashSet<char> rightCompartment) => leftCompartment.Intersect(rightCompartment).Single();

    private static IEnumerable<(HashSet<char> left, HashSet<char> right)> GetBackpacks(string inventory) => (inventory ?? string.Empty).Split(Environment.NewLine, StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries).Select(MakeBackpack);

    private static (HashSet<char> left, HashSet<char> right) MakeBackpack(string backpackContents)
    {
        if (string.IsNullOrWhiteSpace(backpackContents))
        {
            throw new ArgumentNullException(nameof(backpackContents));
        }

        var compartmentSize = (backpackContents.Length + (backpackContents.Length % NUM_COMPARTMENTS)) / NUM_COMPARTMENTS;

        var compartments = backpackContents.ToArray().Chunk(compartmentSize).ToArray();

        if (compartments.Length != NUM_COMPARTMENTS)
        {
            throw new Exception($"Error calculating compartments for: '{backpackContents}'");
        }

        return (left: compartments[0].ToHashSet(), right: compartments[1].ToHashSet());
    }
}
