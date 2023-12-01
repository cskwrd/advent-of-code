using AOC.Tests.Common;
using AOC.TwentyThree;
using FluentAssertions;

namespace AOC.Tests.TwentyThree;
public class Day_1_Tests : TestBase
{
    //https://adventofcode.com/2023/day/1
    
    const string EXAMPLE_1_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyThree.Day1.Example1.input.txt";
    const string EXAMPLE_2_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyThree.Day1.Example2.input.txt";
    const string PUZZLE_INPUT = "AOC.Tests.Resources.Input.TwoThousand.TwentyThree.Day1.Puzzle.input.txt";
    
    [Fact]
    public async Task Part1Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_1_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("142");
    }
    
    [Fact]
    public async Task Part1()
    {
        var input = await ReadFromResourceAsync(PUZZLE_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart1(input);

        answer.Should().Be("54990");
    }

    [Fact]
    public Task WordConvert1()
    {
        const string input = @"zoneight234";
        const string expected = @"Z1IGHT234";
        Day1.ConvertWordsToDigitsStrategy(input).Should().Be(expected);
        return Task.CompletedTask;
    }

    [Fact]
    public Task WordConvert2()
    {
        const string input = @"eightwothree";
        const string expected = @"8WO3";
        Day1.ConvertWordsToDigitsStrategy(input).Should().Be(expected);
        return Task.CompletedTask;
    }
    
    [Fact]
    public async Task Part2Example()
    {
        var input = await ReadFromResourceAsync(EXAMPLE_2_INPUT);

        var sut = new Day1();

        string answer = sut.AnswerPart2(input);

        answer.Should().Be("281");
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
