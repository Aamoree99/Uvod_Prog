use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};
use std::thread;
use std::time::Duration;

// Пауза между упражнениями.
const DELAY_MS: u64 = 700;

// Разделитель для вывода.
const SEPARATOR: &str = "============================================================";

// Определяем задания, для которых требуется Release.
//
// В этих заданиях измеряется скорость выполнения алгоритмов.
// Для новых бенчмарков достаточно добавить имя в match.
fn requires_release(task: &str) -> bool {
    match task {
        "05_04" | "05_06" => true,
        _ => false,
    }
}

// Находим все упражнения в папке src/bin.
//
// Каждое упражнение должно находиться по пути:
// src/bin/05_04/main.rs
//
// Возвращаем отсортированный список названий папок.
fn discover_exercises(bin_dir: &Path) -> io::Result<Vec<String>> {
    let mut tasks = Vec::new();

    for entry in fs::read_dir(bin_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Учитываем только папки, содержащие main.rs.
        if entry.file_type()?.is_dir() && path.join("main.rs").is_file() {
            let name = entry.file_name().into_string().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid exercise directory name",
                )
            })?;

            tasks.push(name);
        }
    }

    // Имена вида 01_01, 05_04 сортируются в правильном порядке.
    tasks.sort();

    Ok(tasks)
}

// Читаем название задания из чешского комментария.
//
// Например:
// Název: Testování rychlostí algoritmů
//
// Ищем название только внутри первого блока комментария.
fn get_task_name(file: &Path) -> io::Result<String> {
    let content = fs::read_to_string(file)?;

    let header = content.trim_start().strip_prefix("/*").unwrap_or("");

    for line in header.split("*/").next().unwrap_or("").lines() {
        if let Some(title) = line.trim().strip_prefix("Název:") {
            return Ok(title.trim().to_string());
        }
    }

    Ok(String::from("Unknown exercise"))
}

// Создаём команду Cargo.
//
// Команда выполняется из корня Rust-проекта.
// Ввод и вывод наследуются от терминала,
// поэтому интерактивные задания продолжают работать.
fn cargo_command(project_dir: &Path) -> Command {
    let cargo = env::var_os("CARGO").unwrap_or_else(|| "cargo".into());

    let mut command = Command::new(cargo);

    command
        .current_dir(project_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    command
}

// Выводим информацию о запускаемом упражнении.
fn print_header(task: &str, title: &str, release: bool) {
    println!();
    println!("{SEPARATOR}");
    println!("Exercise: {task}");
    println!("Title:    {title}");

    if release {
        println!("Mode:     Release");
    } else {
        println!("Mode:     Debug");
    }

    println!("{SEPARATOR}");
    println!();
}

// Компилируем конкретное упражнение.
//
// Для бенчмарков используем Release.
// Для остальных заданий — Debug, если не выбран Release вручную.
//
// Возвращаем true, если компиляция прошла успешно.
fn build_exercise(project_dir: &Path, task: &str, release: bool) -> io::Result<bool> {
    let mut command = cargo_command(project_dir);

    command.args(["build", "--quiet", "--bin", task]);

    if release {
        command.arg("--release");
    }

    Ok(command.status()?.success())
}

// Запускаем упражнение.
//
// cargo run проверяет актуальность сборки,
// поэтому обычно повторная компиляция не требуется.
fn execute_exercise(project_dir: &Path, task: &str, release: bool) -> io::Result<bool> {
    let mut command = cargo_command(project_dir);

    command.args(["run", "--quiet", "--bin", task]);

    if release {
        command.arg("--release");
    }

    Ok(command.status()?.success())
}

// Показываем список всех доступных упражнений.
//
// Рядом с названием отмечаем режим запуска по умолчанию.
fn list_exercises(bin_dir: &Path, tasks: &[String]) -> io::Result<()> {
    println!("{SEPARATOR}");
    println!("AVAILABLE EXERCISES");
    println!("{SEPARATOR}");

    for task in tasks {
        let title = get_task_name(&bin_dir.join(task).join("main.rs"))?;

        let mode = if requires_release(task) {
            "Release"
        } else {
            "Debug"
        };

        println!("{:<10} | {:<7} | {}", task, mode, title);
    }

    println!("{SEPARATOR}");
    println!("Total exercises: {}", tasks.len());

    Ok(())
}

// Показываем справку по аргументам запуска.
fn print_help() {
    println!("Rust Exercise Runner");
    println!();

    println!("Usage:");
    println!("  cargo run");
    println!("  cargo run -- --release");
    println!("  cargo run -- --bin 05_04");
    println!("  cargo run -- --release --bin 05_04");
    println!("  cargo run -- --from 05_01");
    println!("  cargo run -- --list");
    println!("  cargo run -- --help");

    println!();
    println!("Options:");
    println!("  --release       Force Release mode for all exercises");
    println!("  --bin NAME      Run one specific exercise");
    println!("  --from NAME     Run exercises starting from NAME");
    println!("  --list          List available exercises");
    println!("  --help          Show this help");

    println!();
    println!("Automatic Release exercises:");
    println!("  05_04          Sorting algorithm benchmark");
    println!("  05_06          Bubble sort benchmark");
}

// Основная функция runner.
//
// Читает аргументы командной строки,
// определяет нужные задания,
// выбирает Debug или Release,
// компилирует и запускает упражнения.
fn run_exercises() -> io::Result<ExitCode> {
    // Получаем путь к корню проекта.
    // Он не зависит от директории, из которой запущен runner.
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let bin_dir = project_dir.join("src/bin");

    // Находим доступные задания.
    let tasks = discover_exercises(&bin_dir)?;

    // Параметры запуска.
    let mut release = false;
    let mut selected_task: Option<String> = None;
    let mut start_from: Option<String> = None;
    let mut list_only = false;

    // Читаем аргументы после имени программы.
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--release" => {
                // Принудительно включаем Release для всех заданий.
                release = true;
            }

            "--bin" => {
                // Выбираем одно конкретное упражнение.
                selected_task = Some(args.next().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Missing exercise name after --bin",
                    )
                })?);
            }

            "--from" => {
                // Выбираем начальное упражнение.
                start_from = Some(args.next().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Missing exercise name after --from",
                    )
                })?);
            }

            "--list" => {
                // Только показать список заданий.
                list_only = true;
            }

            "--help" | "-h" => {
                print_help();
                return Ok(ExitCode::SUCCESS);
            }

            _ => {
                eprintln!("Unknown argument: {arg}");
                print_help();
                return Ok(ExitCode::FAILURE);
            }
        }
    }

    // Если запрошен список, ничего не компилируем.
    if list_only {
        list_exercises(&bin_dir, &tasks)?;
        return Ok(ExitCode::SUCCESS);
    }

    // Проверяем существование выбранного задания.
    if let Some(ref name) = selected_task {
        if tasks.contains(name) == false {
            eprintln!("Exercise not found: {name}");
            return Ok(ExitCode::FAILURE);
        }
    }

    // Проверяем существование начального задания.
    if let Some(ref name) = start_from {
        if tasks.contains(name) == false {
            eprintln!("Starting exercise not found: {name}");
            return Ok(ExitCode::FAILURE);
        }
    }

    // Нельзя одновременно использовать --bin и --from.
    if selected_task.is_some() && start_from.is_some() {
        eprintln!("Cannot use --bin and --from together.");
        return Ok(ExitCode::FAILURE);
    }

    println!();
    println!("{SEPARATOR}");
    println!("RUST EXERCISE RUNNER");

    // Отображаем общий режим работы runner.
    // При автоматическом выборе режим определяется для каждого задания.
    println!(
        "Mode: {}",
        if release {
            "Release (forced)"
        } else {
            "Automatic (Debug / Release)"
        }
    );

    println!("{SEPARATOR}");

    // Счётчики успешно выполненных и завершившихся ошибкой программ.
    let mut completed = 0;
    let mut runtime_errors = 0;

    // Последовательно обрабатываем найденные упражнения.
    for task in tasks {
        // Если выбрано конкретное задание,
        // пропускаем все остальные.
        if let Some(ref selected) = selected_task {
            if &task != selected {
                continue;
            }
        }

        // Если указано --from,
        // пропускаем задания с меньшими номерами.
        if let Some(ref start) = start_from {
            if &task < start {
                continue;
            }
        }

        // Получаем чешское название упражнения.
        let title = get_task_name(&bin_dir.join(&task).join("main.rs"))?;

        // АВТОМАТИЧЕСКИЙ ВЫБОР РЕЖИМА.
        //
        // release == true:
        //   пользователь вручную включил Release.
        //
        // requires_release(&task) == true:
        //   задание находится в списке бенчмарков.
        //
        // В остальных случаях используется Debug.
        let task_release = release || requires_release(&task);

        // Показываем реальный режим конкретного упражнения.
        print_header(&task, &title, task_release);

        thread::sleep(Duration::from_millis(DELAY_MS));

        // Компилируем в выбранном режиме.
        if build_exercise(project_dir, &task, task_release)? == false {
            eprintln!("COMPILE ERROR: {task}");
            return Ok(ExitCode::FAILURE);
        }

        // Очищаем буфер вывода перед запуском программы.
        io::stdout().flush()?;

        // Запускаем в том же режиме, в котором компилировали.
        if execute_exercise(project_dir, &task, task_release)? == false {
            eprintln!("RUNTIME ERROR: {task}");
            runtime_errors += 1;
        } else {
            completed += 1;
        }

        thread::sleep(Duration::from_millis(DELAY_MS));
    }

    // Итоговая статистика.
    println!();
    println!("{SEPARATOR}");
    println!("EXECUTION SUMMARY");
    println!("Completed successfully: {completed}");
    println!("Runtime errors:        {runtime_errors}");
    println!("{SEPARATOR}");

    // Если хотя бы одно упражнение завершилось с ошибкой,
    // runner тоже возвращает код ошибки.
    if runtime_errors > 0 {
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}

// Точка входа.
//
// Возвращаем код завершения программы:
// SUCCESS — выполнение успешное.
// FAILURE — возникла ошибка.
fn main() -> ExitCode {
    match run_exercises() {
        Ok(status) => status,

        Err(error) => {
            eprintln!("RUNNER ERROR: {error}");
            ExitCode::FAILURE
        }
    }
}
