using AOC.Tests.Common;
using FluentAssertions;

namespace AOC.Tests.TwentyTwo;
public class Day_1_Tests : TestBase
{
    [Fact]
    public void Test1()
    {
        var input = ReadFromResource("AOC.Tests.Resources.Input.TwoThousand.TwentyTwo.Day1.Part1Example.txt");
        input.Should().NotBeNullOrEmpty();
    }
}
