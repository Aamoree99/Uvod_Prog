using System.Diagnostics;

// Ищем проект от текущей папки и от каталога исполняемого файла.
static string FindProject()
{
    foreach (string start in new[] {Directory.GetCurrentDirectory(), AppContext.BaseDirectory})
    {
        DirectoryInfo? directory = new(start);
        while (directory != null)
        {
            if (File.Exists(Path.Combine(directory.FullName, "Uvod.csproj"))) return directory.FullName;
            string nested = Path.Combine(directory.FullName, "csharp");
            if (File.Exists(Path.Combine(nested, "Uvod.csproj"))) return nested;
            directory = directory.Parent;
        }
    }
    throw new DirectoryNotFoundException("Cannot find the C# project");
}

// Читаем оригинальное название только до конца заголовка.
static string ExerciseTitle(string source)
{
    string header = File.ReadAllText(source).Split("*/", 2)[0];
    foreach (string line in header.Split('\n'))
        if (line.Trim().StartsWith("Název:")) return line.Trim()["Název:".Length..].Trim();
    return "Unknown exercise";
}

// Без перенаправления потоков дочерняя программа читает ввод прямо из терминала.
static int RunCommand(string project, params string[] arguments)
{
    ProcessStartInfo start = new("dotnet") {WorkingDirectory = project, UseShellExecute = false};
    foreach (string argument in arguments) start.ArgumentList.Add(argument);
    using Process process = Process.Start(start) ?? throw new IOException("Cannot start dotnet");
    process.WaitForExit();
    return process.ExitCode;
}

try
{
    string project = args.Length > 0 ? Path.GetFullPath(args[0]) : FindProject();
    string[] exercises = Directory.GetDirectories(Path.Combine(project, "src", "exercises"));
    Array.Sort(exercises, StringComparer.Ordinal);
    foreach (string exercise in exercises)
    {
        string source = Path.Combine(exercise, "Program.cs");
        string exerciseProject = Path.Combine(exercise, "Exercise.csproj");
        if (!File.Exists(source) || !File.Exists(exerciseProject)) continue;
        Console.WriteLine("\n============================================================");
        Console.WriteLine("Exercise: " + Path.GetFileName(exercise));
        Console.WriteLine("Title: " + ExerciseTitle(source));
        Console.WriteLine("============================================================");
        if (RunCommand(project, "build", exerciseProject, "--nologo", "--verbosity", "quiet") != 0)
        {
            Console.Error.WriteLine("COMPILE ERROR");
            return 1;
        }
        if (RunCommand(project, "run", "--no-build", "--project", exerciseProject) != 0)
            Console.Error.WriteLine("RUNTIME ERROR");
    }
    return 0;
}
catch (Exception error)
{
    Console.Error.WriteLine("RUNNER ERROR: " + error.Message);
    return 1;
}
