using AOC.Tests.Common;
using AOC.TwentyTwo;
using FluentAssertions;

namespace AOC.Tests.TwentyTwo;
public class Day_1_Tests : TestBase
{
    const string EXAMPLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyTwo.Day1.Example.input.txt";
    const string PUZZLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyTwo.Day1.Puzzle.input.txt";
    
    [Fact]
    public async Task Part1Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("24000");
    }
    
    [Fact]
    public async Task Part1()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("72478");
    }
    
    [Fact]
    public async Task Part2Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("45000");
    }
    
    [Fact]
    public async Task Part2()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("210367");
    }
}
