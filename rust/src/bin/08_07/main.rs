/*
Cvičení 08 — Úloha 8.7
Název: Výpis souborů
Body: 3 body

Zadání:
Napište funkci, která do zadaného textového souboru vypíše názvy všech souborů nacházejících se
v zadaném adresáři a obsahujících zadané slovo. Pokud se soubor určený pro výpis výsledků nachází
v adresáři, ve kterém hledáme soubory, je ve výpisu všech souborů uveden i tento soubor určený pro
výpis výsledků. [3 body]


Ukázka:

VypisSouboru("vypis.txt", ".", "posledni");

// C:\UPg\Hodina 08\c07 Vypis souboru\bin\Debug\aaa.txt

Požadavky / omezení:
a) K řešení příkladů je možné používat pouze syntaktických konstrukcí a funkcí probraných na
přednáškách. K řešení cvičení s nižším číslem nelze používat učivo hodin s vyšším číslem.

b) Řešení musí být procedurální, nesmí být objektové. Je zakázáno použití cyklu foreach. Je
zakázáno použití kolekcí (Dictionary, List, LinkedList, SortedList, atd.) a jejich obdoby v jiných
programovacích jazycích.

c) Příklady můžete řešit i v jiném jazyku než C#, pokud tento jazyk umožňuje procedurální
programování, např. C, C++, Java. Při této volbě zdůrazňuji platnost pravidla 2a, lze používat
pouze ekvivalenty syntaktických konstrukcí a funkcí probraných na přednášce.

d) Úloha musí být funkční. Otestujte ji minimálně na datech uvedených v zadání příkladu.

e) Zdrojové kódy musí být formátovány jednotným způsobem, zejména odsazování.

f) Proměnné a funkce musejí mít vhodné názvy

g) Zdrojové kódy musí být vhodně členěny do funkcí (platí pro 3. cvičení a následující)

h) Pro každé cvičení vytvořte vlastní Solution (Rešení). Jednotlivé příklady budou jako samostatné
Projekty.

i) Je zakázáno používat složené příkazy { }, pokud obsahují pouze jeden příkaz.
*/

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// --------------------------------------------------
// A) CHECK FILE CONTENT
// --------------------------------------------------

// Проверяем, содержит ли файл заданный текст.
//
// Читаем файл целиком как UTF-8.
// Если файл невозможно прочитать как текст,
// считаем, что совпадение не найдено.
fn file_contains_text(filename: &Path, search_text: &str) -> bool {
    // Пробуем прочитать содержимое файла.
    let content = fs::read_to_string(filename);

    // Если файл прочитан успешно, ищем подстроку.
    match content {
        Ok(text) => text.contains(search_text),
        Err(_) => false,
    }
}

// --------------------------------------------------
// B) LIST FILES
// --------------------------------------------------

// Ищем файлы в заданной директории,
// содержимое которых включает search_text.
//
// output_filename — файл для списка результатов.
// directory — директория поиска.
// search_text — искомая подстрока.
//
// Поиск выполняется только в самой директории,
// без обхода вложенных каталогов.
//
// Выходной файл включаем в список отдельно,
// если он расположен в директории поиска.
fn list_files(output_filename: &str, directory: &str, search_text: &str) {
    assert!(
        search_text.is_empty() == false,
        "Search text cannot be empty"
    );

    // Получаем абсолютный путь директории.
    let directory_path = fs::canonicalize(directory).expect("Cannot access directory");

    // Определяем полный путь выходного файла.
    let output_path = Path::new(output_filename);

    // Создаём выходной файл до начала поиска.
    // Это нужно, чтобы он уже существовал
    // при перечислении файлов директории.
    let mut output = File::create(output_path).expect("Cannot create output file");

    // Получаем абсолютный путь выходного файла.
    let output_absolute = fs::canonicalize(output_path).expect("Cannot resolve output file path");

    // Открываем итератор элементов директории.
    //
    // Здесь не создаём коллекцию файлов:
    // read_dir() возвращает записи по одной.
    let mut entries = fs::read_dir(&directory_path).expect("Cannot read directory");

    let mut files_checked = 0;
    let mut files_matched = 0;

    // Проверяем элементы директории по одному.
    while let Some(entry_result) = entries.next() {
        let entry = entry_result.expect("Cannot read directory entry");

        let path = entry.path();

        // Пропускаем директории и другие объекты.
        if path.is_file() == false {
            continue;
        }

        files_checked += 1;

        // Преобразуем путь к абсолютному.
        let absolute_path = fs::canonicalize(&path).expect("Cannot resolve file path");

        // Проверяем, является ли файл выходным.
        let is_output = absolute_path == output_absolute;

        // Выходной файл включаем независимо
        // от его содержимого.
        //
        // Для остальных файлов ищем текст.
        let matches = if is_output {
            true
        } else {
            file_contains_text(&path, search_text)
        };

        if matches {
            // Записываем полный путь в файл.
            writeln!(output, "{}", absolute_path.display()).expect("Cannot write file path");

            files_matched += 1;
        }
    }

    // Завершаем запись.
    output.flush().expect("Cannot flush output file");

    println!("Directory: {}", directory_path.display());
    println!("Search text: {}", search_text);
    println!("Output file: {}", output_absolute.display());
    println!("Files checked: {}", files_checked);
    println!("Files matched: {}", files_matched);
}

// --------------------------------------------------
// C) CREATE SAMPLE FILE
// --------------------------------------------------

// Создаём демонстрационный файл,
// чтобы можно было проверить поиск.
//
// Если файл уже существует, не перезаписываем его.
fn create_sample_file(filename: &str) {
    if Path::new(filename).exists() {
        return;
    }

    let mut file = File::create(filename).expect("Cannot create sample file");

    writeln!(file, "Tohle je posledni radek souboru.").expect("Cannot write sample");

    writeln!(file, "Tento soubor slouzi k testovani.").expect("Cannot write sample");
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    let output_filename = "vypis.txt";
    let directory = ".";
    let search_text = "posledni";

    println!("FILE CONTENT SEARCH");
    println!("========================================");

    // Создаём пример файла с искомым текстом.
    create_sample_file("sample_08_07.txt");

    // Выполняем поиск.
    list_files(output_filename, directory, search_text);

    println!("Search completed successfully.");

    // Показываем созданный список.
    println!();
    println!("MATCHING FILES");
    println!("----------------------------------------");

    let result = fs::read_to_string(output_filename).expect("Cannot read output file");

    print!("{}", result);
}
