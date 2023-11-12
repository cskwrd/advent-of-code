
using System.Runtime.CompilerServices;

namespace AOC.TwentyTwo;
public class Day2 : DayBase
{
    public override string AnswerPart1(string input) => DecryptStrategyGuideViaDirectResponse(input).Select(decryptedPage => CalculateScoreForRound(decryptedPage.call, decryptedPage.response)).Sum().ToString();

    public override string AnswerPart2(string input) => DecryptStrategyGuideViaCalculatedResponse(input).Select(decryptedPage => CalculateScoreForRound(decryptedPage.call, decryptedPage.response)).Sum().ToString();

    private static int CalculateScoreForRound(Moves call, Moves response)
    {
        if (call == Moves.Rock)
        {
            if (response == Moves.Rock)
            {
                return 4; // draw + rock
            }
            else if (response == Moves.Paper)
            {
                return 8; // win + paper
            }
            else if (response == Moves.Scissors)
            {
                return 3; // lose + scissors
            }
        }
        else if (call == Moves.Paper)
        {
            if (response == Moves.Rock)
            {
                return 1; // lose + rock
            }
            else if (response == Moves.Paper)
            {
                return 5; // draw + paper
            }
            else if (response == Moves.Scissors)
            {
                return 9; // win + scissors
            }
        }
        else if (call == Moves.Scissors)
        {
            if (response == Moves.Rock)
            {
                return 7; // win + rock
            }
            else if (response == Moves.Paper)
            {
                return 2; // lose + paper
            }
            else if (response == Moves.Scissors)
            {
                return 6; // draw + scissors
            }
        }
        throw new Exception($"Problem tallying score for: '{call}' x '{response}'");
    }

    private static IEnumerable<(Moves call, Moves response)> DecryptStrategyGuideViaDirectResponse(string strategyGuide)
    {
        if (string.IsNullOrWhiteSpace(strategyGuide) == false)
        {
            var encryptedPages = strategyGuide.Split(Environment.NewLine, StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
            foreach (var encryptedPage in encryptedPages)
            {
                var encryptedPageContent = encryptedPage.Split(' ', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
                var encryptedCall = encryptedPageContent[0].ToUpperInvariant();
                var encryptedResponse = encryptedPageContent[1].ToUpperInvariant();
                var decryptedCall = encryptedCall switch
                {
                    "A" => Moves.Rock,
                    "B" => Moves.Paper,
                    "C" => Moves.Scissors,
                    _ => throw new Exception($"Unable to decrypt call: '{encryptedCall}'"),
                };
                var decryptedResponse = encryptedResponse switch
                {
                    "X" => Moves.Rock,
                    "Y" => Moves.Paper,
                    "Z" => Moves.Scissors,
                    _ => throw new Exception($"Unable to decrypt response: '{encryptedResponse}'"),
                };
                yield return (call: decryptedCall, response: decryptedResponse);
            }
        }
    }

    private static IEnumerable<(Moves call, Moves response)> DecryptStrategyGuideViaCalculatedResponse(string strategyGuide)
    {
        if (string.IsNullOrWhiteSpace(strategyGuide) == false)
        {
            var encryptedPages = strategyGuide.Split(Environment.NewLine, StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
            foreach (var encryptedPage in encryptedPages)
            {
                var encryptedPageContent = encryptedPage.Split(' ', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
                var encryptedCall = encryptedPageContent[0].ToUpperInvariant();
                var encryptedResult = encryptedPageContent[1].ToUpperInvariant();
                var decryptedCall = encryptedCall switch
                {
                    "A" => Moves.Rock,
                    "B" => Moves.Paper,
                    "C" => Moves.Scissors,
                    _ => throw new Exception($"Unable to decrypt call: '{encryptedCall}'"),
                };
                var response = encryptedResult switch
                {
                    "X" => Moves.Rock,
                    "Y" => Moves.Paper,
                    "Z" => Moves.Scissors,
                    _ => throw new Exception($"Unable to decrypt response: '{encryptedResult}'"),
                };
                if (decryptedCall == Moves.Rock)
                {
                    if (encryptedResult == "X")
                    {
                        yield return (call: decryptedCall, response: Moves.Scissors); // lose
                        continue;
                    }
                    else if (encryptedResult == "Y")
                    {
                        yield return (call: decryptedCall, response: Moves.Rock); // draw
                        continue;
                    }
                    else if (encryptedResult == "Z")
                    {
                        yield return (call: decryptedCall, response: Moves.Paper); // win
                        continue;
                    }
                }
                else if (decryptedCall == Moves.Paper)
                {
                    if (encryptedResult == "X")
                    {
                        yield return (call: decryptedCall, response: Moves.Rock); // lose
                        continue;
                    }
                    else if (encryptedResult == "Y")
                    {
                        yield return (call: decryptedCall, response: Moves.Paper); // draw
                        continue;
                    }
                    else if (encryptedResult == "Z")
                    {
                        yield return (call: decryptedCall, response: Moves.Scissors); // win
                        continue;
                    }
                }
                else if (decryptedCall == Moves.Scissors)
                {
                    if (encryptedResult == "X")
                    {
                        yield return (call: decryptedCall, response: Moves.Paper); // lose
                        continue;
                    }
                    else if (encryptedResult == "Y")
                    {
                        yield return (call: decryptedCall, response: Moves.Scissors); // draw
                        continue;
                    }
                    else if (encryptedResult == "Z")
                    {
                        yield return (call: decryptedCall, response: Moves.Rock); // win
                        continue;
                    }
                }
                throw new Exception("123"); // too lazy to make a good msg
            }
        }
    }

    private enum Moves
    {
        Rock,
        Paper,
        Scissors,
    }
}
