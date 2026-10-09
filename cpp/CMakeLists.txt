cmake_minimum_required(VERSION 3.16)
project(uvod_cpp LANGUAGES CXX)

add_executable(uvod_cpp src/main.cpp)
target_compile_features(uvod_cpp PRIVATE cxx_std_17)
target_compile_definitions(uvod_cpp PRIVATE UVOD_SOURCE_DIR="${CMAKE_CURRENT_SOURCE_DIR}")

# Каждая готовая задача — отдельный исполняемый файл с тем же номером, что в Rust.
file(GLOB exercise_sources CONFIGURE_DEPENDS "src/exercises/*/main.cpp")
foreach(exercise_source IN LISTS exercise_sources)
    get_filename_component(exercise_dir "${exercise_source}" DIRECTORY)
    get_filename_component(exercise_name "${exercise_dir}" NAME)
    add_executable("${exercise_name}" "${exercise_source}")
    target_compile_features("${exercise_name}" PRIVATE cxx_std_17)
endforeach()
