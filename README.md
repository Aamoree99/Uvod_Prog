# Úvod do programování

## 1. О проекте

Учебные задания по предмету **Úvod do programování** на Rust, C#, C++, Java и Python.
Оригинальные чешские условия находятся в начале исходников; полный сборник —
[`UPg_cviceni.txt`](UPg_cviceni.txt). Готовность языков различается: см. таблицу ниже.

## 2. Структура проекта

```text
uvod/
├── rust/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                         # общий runner
│       └── bin/03_06/main.rs               # отдельное задание
├── csharp/
│   ├── Uvod.csproj                        # проект runner
│   └── src/
│       ├── Program.cs                     # общий runner
│       └── exercises/03_06/
│           ├── Exercise.csproj            # проект задания
│           └── Program.cs
├── cpp/
│   ├── CMakeLists.txt
│   └── src/
│       ├── main.cpp                       # общий runner
│       └── exercises/03_06/main.cpp
├── java/
│   └── src/main/java/
│       ├── Main.java                      # общий runner
│       └── exercises/task_03_06/Main.java
├── python/
│   ├── main.py                            # общий runner
│   ├── exercises/03_06/main.py             # отдельное задание
│   └── tests/test_exercises.py             # проверки алгоритмов
├── zapocet/
│   └── README.md                          # описание тестового задания E
└── UPg_cviceni.txt
```

`03_06` в дереве — пример пути. Остальные задания расположены аналогично;
`zapocet_E` в Java находится в пакете `exercises.task_zapocet_E`.

## 3. Нумерация заданий

`03_06` означает **Cvičení 03, Úloha 3.6**. Нули обеспечивают порядок номеров.
`zapocet_E` — отдельное учебное задание к zápočtu.

## 4. Запуск Rust

Нужны Rust и Cargo с поддержкой edition 2024 (Rust 1.85+).

```bash
cd rust
cargo run --bin 03_06          # прямой запуск задания
cargo run                      # runner: все задания по порядку
cargo run -- --list             # номера, чешские названия и режимы
cargo run -- --bin 05_04        # одно задание через runner
cargo run -- --from 05_01       # начиная с указанного задания
```

Runner автоматически выбирает Release для бенчмарков `05_04` и `05_06`,
Debug — для остальных. `cargo run -- --release` принудительно включает Release.
Прямой `cargo run --bin ...` не использует эту логику: для Release нужно
явно добавить `--release`. Runner запускает также существующие заглушки.

## 5. Запуск C#

Нужен .NET SDK 8. Команды из корня репозитория:

```bash
dotnet run --project csharp/src/exercises/03_06/Exercise.csproj
dotnet run --project csharp/Uvod.csproj   # runner: все доступные задания
```

## 6. Запуск C++

Нужны CMake 3.16+ и компилятор C++17. Для Unix Makefiles на Linux/macOS:

```bash
cd cpp
cmake -S . -B build -G "Unix Makefiles"
cmake --build build --target 03_06
./build/03_06
```

Путь бинарника — `cpp/build/03_06` относительно корня репозитория.
Общий runner: `cmake --build build --target uvod_cpp`, затем `./build/uvod_cpp`.
CMake должен быть доступен в PATH и при запуске runner.
Для генераторов с несколькими конфигурациями путь будет отличаться.

## 7. Запуск Java

Нужен JDK 17+ с `javac` и `java`. Команды из корня репозитория:

```bash
cd java
mkdir -p build/classes
javac -encoding UTF-8 -d build/classes src/main/java/exercises/task_03_06/Main.java
java -cp build/classes exercises.task_03_06.Main
# Общий runner:
javac -encoding UTF-8 -d build/classes src/main/java/Main.java
java -cp build/classes Main
```

Имя класса включает package `exercises.task_XX_YY`, а не только `Main`.

## 8. Запуск Python

Нужен Python **3.11+**; внешних зависимостей нет.

```bash
cd python
python3 exercises/03_06/main.py  # прямой интерактивный запуск
python3 main.py --bin 03_06
python3 main.py --list
python3 main.py                 # все задания, включая заглушки
python3 main.py --from 05_01
python3 main.py --help
python3 -m unittest discover -s tests -v
```

Runner находит задания автоматически, показывает номера и чешские названия,
запускает их отдельными процессами с терминальным вводом/выводом и сообщает
статистику решений, заглушек и ошибок. Путь к упражнениям определяется от
`main.py`: можно запускать его по абсолютному пути из любой директории.
Рулетка `06_04` завершается командой `quit`; последовательности — числом ≤ 0.
Бенчмарки используют обычный Python и `time.perf_counter()` только вокруг
сортировки. Все алгоритмы одного сравнения получают копии одинакового массива.
Размеры 1000, 10000 и 30000 сохранены; большие тесты могут занимать несколько минут.

## 9. Состояние решений

Количество проверено по исходникам, включая `zapocet_E`:

| Язык | Представлено | Реализовано | Заглушки |
|---|---:|---:|---:|
| Rust | 78 | 55 | 23 |
| C# | 38 | 38 | 0 |
| C++ | 38 | 38 | 0 |
| Java | 38 | 38 | 0 |
| Python | 78 | 55 | 23 |

Rust и Python: готово `01_01–07_05` и `zapocet_E`; задания 08–12 — заглушки.
C#, C++ и Java: готово `01_01–04_08` и `zapocet_E`; более поздних файлов нет.
В `zapocet/` находится только описание задания E: найти второе максимальное
число. Реализации находятся в каталогах языков; полных вариантов тестов нет.
«Реализовано» означает наличие алгоритма, а не подтверждение прохождения всех
проверок на каждом языке. Проверки и исправленные расхождения Python описаны
в [`python/NOTES.md`](python/NOTES.md).
