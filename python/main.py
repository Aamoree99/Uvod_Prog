"""Runner для учебных упражнений Úvod do programování (Python 3.11+)."""

import argparse
from pathlib import Path
import subprocess
import sys

EXERCISES = Path(__file__).resolve().parent / "exercises"
SEPARATOR = "=" * 60


def exercise_key(path):
    parts = path.name.split("_")
    if len(parts) == 2 and parts[0].isdigit() and parts[1].isdigit():
        return 0, int(parts[0]), int(parts[1]), ""
    return 1, 0, 0, path.name


def discover_exercises(directory):
    return sorted(
        (path for path in directory.iterdir() if path.is_dir() and (path / "main.py").is_file()),
        key=exercise_key,
    )


def exercise_info(path):
    source = (path / "main.py").read_text(encoding="utf-8")
    header = source.split('"""', 2)[1]
    title = "Unknown exercise"
    for line in header.splitlines():
        if line.strip().startswith("Název:"):
            title = line.strip().removeprefix("Název:").strip()
            break
    stub = 'print("This exercise is not implemented yet.")' in source
    return title, stub


def main():
    parser = argparse.ArgumentParser(description="Python Exercise Runner")
    parser.add_argument("--list", action="store_true", help="List available exercises")
    selection = parser.add_mutually_exclusive_group()
    selection.add_argument("--bin", metavar="NAME", help="Run one exercise")
    selection.add_argument("--from", dest="start_from", metavar="NAME", help="Run exercises starting from NAME")
    args = parser.parse_args()
    try:
        tasks = discover_exercises(EXERCISES)
        names = [path.name for path in tasks]
        for name in (args.bin, args.start_from):
            if name is not None and name not in names:
                parser.error(f"Exercise not found: {name}")
        if args.list:
            for path in tasks:
                title, stub = exercise_info(path)
                print(f"{path.name:<10} | {'STUB' if stub else 'IMPLEMENTED':<11} | {title}")
            print(f"Total exercises: {len(tasks)}")
            return 0
        if args.bin:
            tasks = [path for path in tasks if path.name == args.bin]
        elif args.start_from:
            tasks = tasks[names.index(args.start_from):]
        completed = runtime_errors = stubs = 0
        print("PYTHON EXERCISE RUNNER", flush=True)
        try:
            for path in tasks:
                title, stub = exercise_info(path)
                print(f"\n{SEPARATOR}\nExercise: {path.name}\nTitle: {title}\nStatus: {'STUB' if stub else 'IMPLEMENTED'}\n{SEPARATOR}", flush=True)
                # Потоки наследуются от терминала; каждый файл запускается отдельно.
                try:
                    result = subprocess.run([sys.executable, str(path / "main.py")], cwd=EXERCISES.parent)
                except OSError as error:
                    print(f"RUNTIME ERROR: {path.name}: {error}", file=sys.stderr)
                    runtime_errors += 1
                    continue
                if result.returncode != 0:
                    print(f"RUNTIME ERROR: {path.name} (exit code {result.returncode})", file=sys.stderr)
                    runtime_errors += 1
                elif stub:
                    stubs += 1
                else:
                    completed += 1
        except KeyboardInterrupt:
            print("\nExecution interrupted.", file=sys.stderr)
            runtime_errors += 1
        print(f"\n{SEPARATOR}\nEXECUTION SUMMARY\nCompleted successfully: {completed}\nStubs executed: {stubs}\nRuntime errors: {runtime_errors}\n{SEPARATOR}")
        return 1 if runtime_errors else 0
    except (OSError, ValueError, IndexError) as error:
        print(f"RUNNER ERROR: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
