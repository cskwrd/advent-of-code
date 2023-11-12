using AOC.Tests.Common;
using AOC.TwentyTwo;
using FluentAssertions;

namespace AOC.Tests.TwentyTwo;
public class Day_2_Tests : TestBase
{
    //https://adventofcode.com/2022/day/2
    
    const string EXAMPLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyTwo.Day2.Example.input.txt";
    const string PUZZLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyTwo.Day2.Puzzle.input.txt";
    
    [Fact]
    public async Task Part1Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day2();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("15");
    }
    
    [Fact]
    public async Task Part1()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day2();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("11873");
    }
    
    [Fact]
    public async Task Part2Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day2();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("12");
    }
    
    [Fact]
    public async Task Part2()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day2();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("12014");
    }
}
