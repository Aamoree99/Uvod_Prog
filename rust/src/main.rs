use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};
use std::thread;
use std::time::Duration;

// Cargo обнаруживает программы в src/bin/<имя>/main.rs.
// Читаем только этот уровень: запускатель и каталог target сюда не попадают.
fn discover_exercises(bin_dir: &Path) -> io::Result<Vec<String>> {
    let mut tasks = Vec::new();
    for entry in fs::read_dir(bin_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() && entry.path().join("main.rs").is_file() {
            let name = entry.file_name().into_string().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Invalid exercise directory name",
                )
            })?;
            tasks.push(name);
        }
    }
    // Номера записаны с ведущими нулями, поэтому обычная сортировка даёт нужный порядок.
    tasks.sort();
    Ok(tasks)
}

fn get_task_name(file: &Path) -> io::Result<String> {
    let content = fs::read_to_string(file)?;
    // Название ищем в начальном комментарии, а не в исполняемом коде.
    let header = content.trim_start().strip_prefix("/*").unwrap_or("");
    for line in header.split("*/").next().unwrap_or("").lines() {
        if let Some(title) = line.trim().strip_prefix("Název:") {
            return Ok(title.trim().to_owned());
        }
    }
    Ok(String::from("Neznama uloha"))
}

fn cargo_command(project_dir: &Path) -> Command {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = Command::new(cargo);
    command
        .current_dir(project_dir)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    command
}

fn run_exercises() -> io::Result<ExitCode> {
    // Путь к проекту не зависит от того, из какой папки запустили программу.
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let bin_dir = project_dir.join("src/bin");
    for task in discover_exercises(&bin_dir)? {
        let title = get_task_name(&bin_dir.join(&task).join("main.rs"))?;
        println!();
        println!("============================================================");
        println!("Uloha: {task}");
        println!("Nazev: {title}");
        println!("============================================================");
        thread::sleep(Duration::from_millis(700));

        // Отдельная сборка отличает ошибку компиляции от сбоя самой программы.
        // Все зависимости, включая rand, разрешает Cargo.
        let compile = cargo_command(project_dir)
            .args(["build", "--quiet", "--bin", &task])
            .status()?;
        if !compile.success() {
            println!("COMPILE ERROR");
            return Ok(ExitCode::FAILURE);
        }

        println!();
        // Потоки наследуются: интерактивные задачи читают ввод прямо из терминала.
        let run = cargo_command(project_dir)
            .args(["run", "--quiet", "--bin", &task])
            .status()?;
        if !run.success() {
            println!("RUNTIME ERROR");
        }
        thread::sleep(Duration::from_millis(700));
    }
    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    match run_exercises() {
        Ok(status) => status,
        Err(error) => {
            eprintln!("RUNNER ERROR: {error}");
            ExitCode::FAILURE
        }
    }
}
