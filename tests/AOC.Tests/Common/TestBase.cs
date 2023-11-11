namespace AOC.Tests.Common;
public class TestBase
{
    public string ReadFromResource(string resourceName)
    {
        Assembly assembly = Assembly.GetExecutingAssembly();

        using (Stream? stream = assembly!.GetManifestResourceStream(resourceName))
        using (StreamReader reader = new(stream!))
        {
            string result = reader.ReadToEnd();
            return result;
        }
    }
}
