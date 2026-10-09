import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;

public class Main {
    // Ищем проект от текущей папки и от расположения скомпилированного класса.
    static Path findProject() throws Exception {
        Path[] starts = {Path.of("").toAbsolutePath(), Path.of(Main.class.getProtectionDomain().getCodeSource().getLocation().toURI())};
        for (Path start : starts) {
            for (Path directory = start; directory != null; directory = directory.getParent()) {
                if (Files.isDirectory(directory.resolve("src/main/java/exercises"))) return directory;
                Path nested = directory.resolve("java");
                if (Files.isDirectory(nested.resolve("src/main/java/exercises"))) return nested;
            }
        }
        throw new IOException("Cannot find the Java project");
    }

    // Заголовок остаётся чешским; извлекаем поле Název: до конца комментария.
    static String exerciseTitle(Path source) throws IOException {
        String header = Files.readString(source, StandardCharsets.UTF_8).split("\\*/", 2)[0];
        for (String line : header.split("\n"))
            if (line.trim().startsWith("Název:")) return line.trim().substring("Název:".length()).trim();
        return "Unknown exercise";
    }

    // inheritIO сохраняет интерактивный ввод и вывод отдельной программы.
    static int runCommand(Path project, String... command) throws Exception {
        return new ProcessBuilder(command).directory(project.toFile()).inheritIO().start().waitFor();
    }

    public static void main(String[] args) {
        try {
            Path project = args.length > 0 ? Path.of(args[0]).toAbsolutePath() : findProject();
            Path exerciseRoot = project.resolve("src/main/java/exercises");
            String[] exercises = exerciseRoot.toFile().list();
            if (exercises == null) throw new IOException("Cannot discover exercises");
            Arrays.sort(exercises);
            Path build = project.resolve("build/classes");
            Files.createDirectories(build);
            String executableDirectory = Path.of(System.getProperty("java.home"), "bin").toString();
            for (String directory : exercises) {
                Path source = exerciseRoot.resolve(directory).resolve("Main.java");
                if (!Files.isRegularFile(source)) continue;
                String number = directory.startsWith("task_") ? directory.substring(5) : directory;
                System.out.println("\n============================================================\nExercise: " + number
                    + "\nTitle: " + exerciseTitle(source) + "\n============================================================");
                if (runCommand(project, Path.of(executableDirectory, "javac").toString(), "-encoding", "UTF-8", "-d", build.toString(), source.toString()) != 0) {
                    System.err.println("COMPILE ERROR");
                    System.exit(1);
                }
                if (runCommand(project, Path.of(executableDirectory, "java").toString(), "-cp", build.toString(), "exercises." + directory + ".Main") != 0)
                    System.err.println("RUNTIME ERROR");
            }
        } catch (Exception error) {
            System.err.println("RUNNER ERROR: " + error.getMessage());
            System.exit(1);
        }
    }
}
