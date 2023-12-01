namespace AOC;

public abstract class DayBase
{

    protected static bool LineIsNotEmpty(string line) => string.IsNullOrWhiteSpace(line) is not true;
    public abstract string AnswerPart1(string input);
    public abstract string AnswerPart2(string input);
}
