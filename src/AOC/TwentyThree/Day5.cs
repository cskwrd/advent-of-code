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
        Almanac almanac = new(blocks, true);
        var locations = almanac.Location;
        var answer = locations.Min().ToString();
        return answer;
    }

    private static long location = -1;
    private static readonly object lockObject = new object();
    public string AnswerPart22(string input)
    {
        var lines = ConvertInputToLines(input);
        var blocks = lines.SelectMany(BlocksFromInputLines());
        var seedRangesStr = blocks.First().First();

        // too lazy to refactor so just gonna showhorn the new input into the old form
        //var seedsParser = String("seeds:").Before(SkipWhitespaces).Then(
        //    Digit.ManyString()
        //        .Select(long.Parse)
        //        .SeparatedAndOptionallyTerminatedAtLeastOnce(Char(' ').AtLeastOnce())
        //    );
        //var seedRanges = seedsParser.ParseOrThrow(seedRangesStr)
        //    .Chunk(2)
        //    .Select(r =>
        //    {
        //        if (r.Length != 2)
        //        {
        //            throw new Exception("bad range");
        //        }
        //        var start = r[0];
        //        var len = r[1];
        //        return new AlmanacSeedRange(start, len);
        //    })
        //    .ToList();
        //List<string[]> seedBlock = [[$"seeds: {string.Join(" ", seedRanges)}"]];

        Almanac2 almanac = new(blocks);
        //Parallel.For(0, long.MaxValue, (currentValue, state) =>
        //{
        //    // Check if the result is already found
        //    if (Interlocked.Read(ref location) != -1)
        //    {
        //        state.Stop();
        //        return;
        //    }

        //    // Replace this condition with your actual search criteria
        //    if (almanac.IsSeedInLocation(currentValue))
        //    {
        //        // Store the result and signal other threads to stop
        //        lock (lockObject)
        //        {
        //            location = currentValue;
        //        }
        //        state.Stop();
        //    }
        //});
        almanac.SeedsToPlant.Where(s => almanac.IsSeedInLocation());
        _ = almanac.IsSeedInLocation(45);
        return location.ToString();
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
        private Dictionary<string, AlmanacMapping>? _from;

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

        public Almanac(IEnumerable<ICollection<string>> almanacBlocks, bool part2 = false)
        {
            var seedData = almanacBlocks.First().First();
            bool isEven = false;
            SeedsToPlant = _seedsParser.ParseOrThrow(seedData).Where(_ => !part2 || (isEven = !isEven)).ToList();

            var maps = almanacBlocks.Skip(1);
            _mappings = maps.Select(MappingFromBlock);
        }

        private Func<long, bool> DoWork()
        {
            bool shouldReturnValue = true;
            return MemoizedFunc;
            bool MemoizedFunc(long seed)
            {
                var r = shouldReturnValue;
                shouldReturnValue = !shouldReturnValue;
                return r;
            }
        }

        private AlmanacMapping MappingFromBlock(ICollection<string> data)
        {
            var keyInfo = data.First().Replace("-to-", " ").Split(' ', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
            var mappings = data.Skip(1).Select(m => _mappingParser.ParseOrThrow(m));
            return new(keyInfo[1], keyInfo[0], mappings);
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

        private Dictionary<string, AlmanacMapping> GetFromMappings()
        {
            Dictionary<string, AlmanacMapping> from = [];
            foreach (var mapping in _mappings)
            {
                if (from.TryAdd(mapping.From, mapping) == false)
                {
                    from[mapping.From] = mapping;
                }
            }
            return from;
        }

        private List<long> Map(ICollection<long> src, string toKey)
        {
            _to ??= GetToMappings();
            var mappings = _to[toKey] ?? throw new Exception($"Missing {toKey} 'TO' mappings");
            var dst = src.Select(s => mappings[s]).ToList();
            return dst;
        }

        private List<long> RevMap(ICollection<long> src, string fromKey)
        {
            _from ??= GetFromMappings();
            var mappings = _from[fromKey] ?? throw new Exception($"Missing {fromKey} 'FROM' mappings");
            var dst = src.Select(s => mappings[s]).ToList();
            return dst;
        }

        private class AlmanacMapping(string to, string from, IEnumerable<AlmanacRange> mappingFuncs)
        {
            private readonly IEnumerable<AlmanacRange> _mappingFuncs = mappingFuncs;
            private readonly Dictionary<long, long> _mappings = [];

            public string To { get; set; } = to;
            public string From { get; set; } = from;

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

    private class Almanac2
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
            select new AlmanacRange(destinationStart, sourceStart, length)
            ).Labelled("mappingParser");

        private readonly IEnumerable<AlmanacMapping> _mappings;
        private readonly IEnumerable<AlmanacSeedRange> _seedData;
        private Dictionary<string, AlmanacMapping>? _to;
        private Dictionary<string, AlmanacMapping>? _from;

        public bool IsSeedInLocation(long location)
        {
            var humidity = RevMap(location, "humidity");
            var temperature = RevMap(humidity, "temperature");
            var light = RevMap(temperature, "light");
            var water = RevMap(light, "water");
            var fertilizer = RevMap(water, "fertilizer");
            var soil = RevMap(fertilizer, "soil");
            var seed = GetSeedForSoil(soil);
            return seed;
        }

        public ICollection<long> SeedsToPlant { get; }
        public bool GetSeedForSoil(long soil) => _seedData.Any(r => r.Check(soil));

        public Almanac2(IEnumerable<ICollection<string>> almanacBlocks)
        {
            //_seedData = _seedsParser.ParseOrThrow(almanacBlocks.First().First())
            //    .Chunk(2)
            //    .Select(r =>
            //    {
            //        if (r.Length != 2)
            //        {
            //            throw new Exception("bad range");
            //        }
            //        var start = r[0];
            //        var len = r[1];
            //        return new AlmanacSeedRange(start, len);
            //    });

            bool isEvenElement = false;
            SeedsToPlant = _seedsParser.ParseOrThrow(almanacBlocks.First().First()).Where(s => isEvenElement = !isEvenElement).ToList();

            var maps = almanacBlocks.Skip(1);
            _mappings = maps.Select(MappingFromBlock);
        }

        private long RevMap(long dst, string fromKey)
        {
            _from ??= GetFromMappings();
            var mappings = _from[fromKey] ?? throw new Exception($"Missing {fromKey} 'FROM' mappings");
            return mappings[dst];
        }

        private Dictionary<string, AlmanacMapping> GetFromMappings()
        {
            Dictionary<string, AlmanacMapping> from = [];
            foreach (var mapping in _mappings)
            {
                if (from.TryAdd(mapping.From, mapping) == false)
                {
                    from[mapping.From] = mapping;
                }
            }
            return from;
        }

        private AlmanacMapping MappingFromBlock(ICollection<string> data)
        {
            var keyInfo = data.First().Replace("-to-", " ").Split(' ', StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries);
            var mappings = data.Skip(1).Select(m => _mappingParser.ParseOrThrow(m));
            return new(keyInfo[1], keyInfo[0], mappings);
        }

        internal bool IsSeedInLocation()
        {
            throw new NotImplementedException();
        }

        private class AlmanacMapping(string to, string from, IEnumerable<AlmanacRange> mappingFuncs)
        {
            private readonly IEnumerable<AlmanacRange> _mappingFuncs = mappingFuncs;
            private readonly Dictionary<long, long> _mappings = [];

            public string To { get; set; } = to;
            public string From { get; set; } = from;

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
                        throw new Exception("failed to add rev mapping");
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

    private class AlmanacSeedRange(long seedNumber, long length)
    {
        private readonly long _seedNumberStart = seedNumber;
        private readonly long _seedNumberEnd = seedNumber + length;

        public bool Check(long input) => input >= _seedNumberStart && input < _seedNumberEnd;
    }
}
