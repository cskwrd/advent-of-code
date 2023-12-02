namespace AOC.Tests.Common;
public class TestBase
{
    public static async Task<string> ReadFromResourceAsync(string resourceName)
    {
        if (string.IsNullOrWhiteSpace(resourceName))
        {
            throw new ArgumentException("Resource name cannot be null", nameof(resourceName));
        }

        Assembly assembly = Assembly.GetExecutingAssembly();

        if (assembly is null)
        {
            throw new Exception("Something bad happened, assembly is null!");
        }

        using var stream = assembly.GetManifestResourceStream(resourceName);

        if (stream is null)
        {
            throw new Exception("Something bad happened, stream is null!");
        }

        using var reader = new StreamReader(stream);

        return await reader.ReadToEndAsync();
    }
}
