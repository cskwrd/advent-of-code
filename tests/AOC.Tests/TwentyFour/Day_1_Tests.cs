using AOC.Tests.Common;
using AOC.TwentyFour;
using FluentAssertions;

namespace AOC.Tests.TwentyFour;
public class Day_1_Tests : TestBase
{
    //https://adventofcode.com/2024/day/1
    
    const string EXAMPLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyFour.Day1.Example1.input.txt";
    const string PUZZLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyFour.Day1.Puzzle.input.txt";
    
    [Fact]
    public async Task Part1Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("11");
    }
    
    [Fact]
    public async Task Part1()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("2375403");
    }
    
    [Fact]
    public async Task Part2Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("31");
    }
    
    [Fact]
    public async Task Part2()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("23082277");
    }
}
