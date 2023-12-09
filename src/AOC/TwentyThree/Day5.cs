using Pidgin;
using static Pidgin.Parser;
using static System.Runtime.InteropServices.JavaScript.JSType;


namespace AOC.TwentyThree;
public class Day5 : DayBase
{
    public override string AnswerPart1(string input)
    {
        var lines = ConvertInputToLines(input);
        var blocks = lines.SelectMany(BlocksFromInputLines());
        Almanac almanac = new(blocks);
        var locations = almanac.Location;
        var answer = locations.Min().ToString();
        return answer;
    }

    public override string AnswerPart2(string input)
    {
        var lines = ConvertInputToLines(input);
        var blocks = lines.SelectMany(BlocksFromInputLines());
        var seedRangesStr = blocks.First().First();

        // too lazy to refactor so just gonna showhorn the new input into the old form
        var seedsParser = String("seeds:").Before(SkipWhitespaces).Then(
            Digit.ManyString()
                .Select(long.Parse)
                .SeparatedAndOptionallyTerminatedAtLeastOnce(Char(' ').AtLeastOnce())
            );
        var seedRanges = seedsParser.ParseOrThrow(seedRangesStr)
            .Chunk(2)
            .SelectMany(r =>
            {
                if (r.Length != 2)
                {
                    throw new Exception("bad range");
                }
                var start = r[0];
                var len = r[1];
                List<long> s = [];
                for (var i = 0; i < len; i++ )
                {
                    s.Add(start + i);
                }
                return s;
            });
        List<string[]> seedBlock = [[$"seeds: {string.Join(" ", seedRanges)}"]];

        Almanac almanac = new(seedBlock.Concat(blocks.Skip(1)));
        var locations = almanac.Location;
        var answer = locations.Min().ToString();
        return answer;
    }

    private class Almanac
    {
        private static readonly Parser<char, IEnumerable<long>> _seedsParser = String("seeds:").Before(SkipWhitespaces).Then(
            Digit.ManyString()
                .Select(long.Parse)
                .SeparatedAndOptionallyTerminatedAtLeastOnce(Char(' ').AtLeastOnce())
            );

        private readonly Parser<char, AlmanacRange> _mappingParser = (
            from destinationStart in Digit.ManyString().Select(long.Parse).Before(SkipWhitespaces)
            from sourceStart in Digit.ManyString().Select(long.Parse).Before(SkipWhitespaces)
            from length in Digit.ManyString().Select(long.Parse).Before(SkipWhitespaces)
            select new AlmanacRange(sourceStart, destinationStart, length)
            ).Labelled("mappingParser");

        private readonly IEnumerable<AlmanacMapping> _mappings;
        private Dictionary<string, AlmanacMapping>? _to;

        public ICollection<long> SeedsToPlant { get; }

        private ICollection<long>? _soil;
        public ICollection<long> Soil
        {
            get
            {
                _soil ??= Map(SeedsToPlant, "soil");
                return _soil;
            }
        }

        private ICollection<long>? _fertilizer;
        public ICollection<long> Fertilizer
        {
            get
            {
                _fertilizer ??= Map(Soil, "fertilizer");
                return _fertilizer;
            }
        }

        private ICollection<long>? _water;
        public ICollection<long> Water
        {
            get
            {
                _water ??= Map(Fertilizer, "water");
                return _water;
            }
        }

        private ICollection<long>? _light;
        public ICollection<long> Light
        {
            get
            {
                _light ??= Map(Water, "light");
                return _light;
            }
        }

        private ICollection<long>? _temperature;
        public ICollection<long> Temperature
        {
            get
            {
                _temperature ??= Map(Light, "temperature");
                return _temperature;
            }
        }

        private ICollection<long>? _humidity;
        public ICollection<long> Humidity
        {
            get
            {
                _humidity ??= Map(Temperature, "humidity");
                return _humidity;
            }
        }

        private ICollection<long>? _location;
        public ICollection<long> Location
        {
            get
            {
                _location ??= Map(Humidity, "location");
                return _location;
            }
        }

        public Almanac(IEnumerable<ICollection<string>> almanacBlocks)
        {
            var seedData = almanacBlocks.First().First();
            SeedsToPlant = _seedsParser.ParseOrThrow(seedData).ToList();

            var maps = almanacBlocks.Skip(1);
            _mappings = maps.Select(MappingFromBlock);
        }

        private AlmanacMapping MappingFromBlock(ICollection<string> data)
        {
            var keyInfo = data.First().Replace("-to-", " ").Split(' ', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
            var mappings = data.Skip(1).Select(m => _mappingParser.ParseOrThrow(m));
            return new(keyInfo[1], mappings);
        }

        private Dictionary<string, AlmanacMapping> GetToMappings()
        {
            Dictionary<string, AlmanacMapping> to = [];
            foreach (var mapping in _mappings)
            {
                if (to.TryAdd(mapping.To, mapping) == false)
                {
                    to[mapping.To] = mapping;
                }
            }
            return to;
        }

        private List<long> Map(ICollection<long> src, string toKey)
        {
            _to ??= GetToMappings();
            var mappings = _to[toKey] ?? throw new Exception($"Missing {toKey} 'TO' mappings");
            var dst = src.Select(s => mappings[s]).ToList();
            return dst;
        }

        private class AlmanacMapping(string to, IEnumerable<AlmanacRange> mappingFuncs)
        {
            private readonly IEnumerable<AlmanacRange> _mappingFuncs = mappingFuncs;
            private readonly Dictionary<long, long> _mappings = [];

            public string To { get; set; } = to;

            public long this[long key]
            {
                get
                {
                    if (_mappings.TryGetValue(key, out long value))
                    {
                        return value;
                    }
                    var mapping = _mappingFuncs.FirstOrDefault(m => m.Check(key));
                    value = mapping?.Map(key) ?? key;
                    if (_mappings.TryAdd(key, value) == false)
                    {
                        throw new Exception("failed to add mapping");
                    }
                    return value;
                }
            }
        }
    }

    private class AlmanacRange(long sourceStart, long destinationStart, long length)
    {
        private readonly long _sourceStart = sourceStart;
        private readonly long _sourceEnd = sourceStart + length;
        private readonly long _adjustment = destinationStart - sourceStart;

        public bool Check(long input) => input >= _sourceStart && input < _sourceEnd;

        public long Map(long input) => input + _adjustment;
    }
}
