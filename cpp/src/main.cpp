#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

namespace fs = std::filesystem;
#ifndef UVOD_SOURCE_DIR
#define UVOD_SOURCE_DIR "cpp"
#endif

// Экранируем пути для командной оболочки, включая пробелы и апострофы.
std::string shell_quote(const std::string& text) {
    std::string quoted = "'";
    for (char character : text) {
        if (character == '\'') quoted += "'\\''";
        else quoted += character;
    }
    return quoted + "'";
}

// Название берём только из исходного чешского комментария.
std::string exercise_title(const fs::path& source) {
    std::ifstream input(source);
    if (!input) throw std::runtime_error("Cannot read exercise source");
    std::string line;
    while (std::getline(input, line)) {
        if (line.find("*/") != std::string::npos) break;
        const std::string field = "Název:";
        if (line.rfind(field, 0) == 0) {
            auto first = line.find_first_not_of(" \t", field.size());
            return first == std::string::npos ? "Unknown exercise" : line.substr(first);
        }
    }
    return "Unknown exercise";
}

// CMake собирает каждую задачу отдельно; запуск наследует терминальные потоки.
int main(int argc, char* argv[]) {
    try {
        fs::path project = argc > 1 ? argv[1] : UVOD_SOURCE_DIR;
        project = fs::absolute(project);
        fs::path build = project / "build";
        std::vector<fs::path> exercises;
        for (const auto& entry : fs::directory_iterator(project / "src/exercises"))
            if (entry.is_directory() && fs::is_regular_file(entry.path() / "main.cpp"))
                exercises.push_back(entry.path());
        std::sort(exercises.begin(), exercises.end());
        const std::string configure = "cmake -S " + shell_quote(project.string()) + " -B " + shell_quote(build.string());
        if (std::system(configure.c_str()) != 0) {
            std::cerr << "CONFIGURATION ERROR\n";
            return 1;
        }
        for (const auto& exercise : exercises) {
            std::string name = exercise.filename().string();
            std::cout << "\n============================================================\nExercise: " << name
                      << "\nTitle: " << exercise_title(exercise / "main.cpp")
                      << "\n============================================================\n" << std::flush;
            std::string compile = "cmake --build " + shell_quote(build.string()) + " --target " + shell_quote(name);
            if (std::system(compile.c_str()) != 0) {
                std::cerr << "COMPILE ERROR\n";
                return 1;
            }
            std::string run = shell_quote((build / name).string());
            if (std::system(run.c_str()) != 0) std::cerr << "RUNTIME ERROR\n";
        }
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "RUNNER ERROR: " << error.what() << '\n';
        return 1;
    }
}
