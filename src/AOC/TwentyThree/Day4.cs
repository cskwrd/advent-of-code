using Pidgin;
using static Pidgin.Parser;


namespace AOC.TwentyThree;
public class Day4 : DayBase
{
    private readonly Parser<char, Scratchcard> _scratchcardParser;

    public Day4()
    {
        var numberList = Digit.ManyString().SeparatedAndOptionallyTerminated(Char(' '));
        _scratchcardParser = (
            from cardId in Letter.ManyString().Before(SkipWhitespaces).Then(Digit.ManyString().Before(Char(':').Before(SkipWhitespaces)))
            from winners in numberList
            from cardNumbers in Char('|').Before(SkipWhitespaces).Then(numberList)
            select new Scratchcard(long.Parse(cardId), winners.Where(num => string.IsNullOrWhiteSpace(num) == false).Select(long.Parse), cardNumbers.Where(num => string.IsNullOrWhiteSpace(num) == false).Select(long.Parse))
        ).Labelled(nameof(Scratchcard));
    }
    public override string AnswerPart1(string input)
    {
        var lines = ConvertInputToLines(input);
        var nonEmptyLines = lines.Where(LineIsNotEmpty);
        var cards = nonEmptyLines.Select(line => _scratchcardParser.ParseOrThrow(line));
        var worth = cards.Sum(c => c.Value).ToString();
        return worth;
    }

    public override string AnswerPart2(string input)
    {
        var lines = ConvertInputToLines(input);
        var nonEmptyLines = lines.Where(LineIsNotEmpty);
        var cards = nonEmptyLines.Select(line => _scratchcardParser.ParseOrThrow(line));
        Dictionary<long, int> dupes = [];
        var numCards = cards.Sum(c =>
        {
            var cardId = c.Id;
            if (dupes.TryGetValue(cardId, out int copies) == false)
            {
                copies = 0;
            }
            copies += 1; // add original card
            var winCount = c.WinCount;
            for (int j = 1; j <= winCount; j++)
            {
                var id = cardId + j;
                if (dupes.TryAdd(id, copies) == false)
                {
                    dupes[id] += copies;
                }
            }
            return copies;
        }).ToString();
        return numCards;
    }

    private class Scratchcard(long id, IEnumerable<long> winningValues, IEnumerable<long> cardValues)
    {
        public long Id { get; set; } = id;
        private readonly IEnumerable<long> _winners = cardValues.Intersect(winningValues);
        private int? _winCount = null;
        public int WinCount { get => _winCount ?? _winners.Count(); }
        public long Value
        {
            get
            {
                if (WinCount > 1)
                {
                    return (long)Math.Pow(2, WinCount - 1);
                }
                else if (WinCount == 1)
                {
                    return 1;
                }
                return 0;
            }
        }
    }
}
