using AOC.Tests.Common;
using AOC.TwentyThree;
using FluentAssertions;

namespace AOC.Tests.TwentyThree;
public class Day_2_Tests : TestBase
{
    //https://adventofcode.com/2023/day/2
    
    const string EXAMPLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyThree.Day2.Example.input.txt";
    const string PUZZLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyThree.Day2.Puzzle.input.txt";
    
    [Fact]
    public async Task Part1Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day2();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("8");
    }
    
    [Fact]
    public async Task Part1()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day2();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("2439");
    }
    
    [Fact]
    public async Task Part2Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day2();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("2286");
    }
    
    [Fact]
    public async Task Part2()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day2();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("63711");
    }
}
