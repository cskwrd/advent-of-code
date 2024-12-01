using AOC.Tests.Common;
using AOC.TwentyThree;
using FluentAssertions;

namespace AOC.Tests.TwentyThree;
public class Day_5_Tests : TestBase
{
    //https://adventofcode.com/2023/day/5
    
    const string EXAMPLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyThree.Day5.Example.input.txt";
    const string PUZZLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyThree.Day5.Puzzle.input.txt";
    
    [Fact(Skip = "WIP")]
    public async Task Part1Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day5();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("35");
    }
    
    [Fact(Skip = "WIP")]
    public async Task Part1()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day5();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("26218");
    }
    
    [Fact(Skip = "WIP")]
    public async Task Part2Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day5();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("46");
    }

    [Fact(Skip = "WIP")]
    public async Task Part2Example2()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_INPUT);

        var sut = new Day5();

        string answer = sut.AnswerPart22(input);

        answer.Should().Be("46");
    }

    [Fact(Skip = "WIP")]
    public async Task Part2()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day5();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("9997537");
    }
}
