using AOC.Tests.Common;
using AOC.TwentyThree;
using FluentAssertions;

namespace AOC.Tests.TwentyThree;
public class Day_4_Tests : TestBase
{
    //https://adventofcode.com/2023/day/4
    
    const string EXAMPLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyThree.Day4.Example.input.txt";
    const string PUZZLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyThree.Day4.Puzzle.input.txt";
    
    [Fact]
    public async Task Part1Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day4();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("13");
    }
    
    [Fact]
    public async Task Part1()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day4();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("26218");
    }
    
    [Fact]
    public async Task Part2Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day4();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("30");
    }
    
    [Fact]
    public async Task Part2()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day4();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("9997537");
    }
}
