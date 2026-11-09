/*
Cvičení 08 — Úloha 8.5
Název: Řádky se zadaným řetězcem
Body: 2 body

Zadání:
Napište funkci, která ze zadaného textového souboru vytvoří nový textový soubor, který bude
obsahovat pouze ty řádky původního souboru, na kterých se nachází zadaný řetězec. Velikost písmen
se ignoruje [2 body]


Ukázka:

RadkySeSlovem("aaa.txt", "novy.txt", "dnes");

/* Dnes je sobota.

 * vcera, dnes, zitra

 * dnesni

 */

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

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// --------------------------------------------------
// A) SEARCH FOR TEXT
// --------------------------------------------------

// Проверяем, содержит ли строка заданный текст.
//
// Регистр букв игнорируется.
//
// Например:
// "Dnes je sobota." и "dnes" -> true
// "Zitra je nedele." и "dnes" -> false
//
// Поиск выполняется по подстроке, а не по целому слову.
// Поэтому "dnesni" тоже подходит.
fn contains_text(line: &str, search_text: &str) -> bool {
    // Переводим обе строки в нижний регистр.
    let lower_line = line.to_lowercase();
    let lower_search = search_text.to_lowercase();

    // Проверяем наличие подстроки.
    lower_line.contains(&lower_search)
}

// --------------------------------------------------
// B) FILTER FILE LINES
// --------------------------------------------------

// Читаем исходный файл построчно.
//
// Если строка содержит заданный текст,
// записываем её в новый файл.
//
// Порядок строк и оригинальный регистр сохраняются.
fn filter_lines(input_filename: &str, output_filename: &str, search_text: &str) {
    // Запрещаем пустую строку поиска.
    assert!(
        search_text.is_empty() == false,
        "Search text cannot be empty"
    );

    // Открываем исходный файл.
    let input = File::open(input_filename).expect("Cannot open input file");

    // Создаём буфер для построчного чтения.
    let mut reader = BufReader::new(input);

    // Создаём выходной файл.
    // Если он уже существует, перезаписываем его.
    let mut output = File::create(output_filename).expect("Cannot create output file");

    // Буфер для текущей строки.
    let mut line = String::new();

    // Счётчики обработанных и найденных строк.
    let mut total_lines = 0;
    let mut matching_lines = 0;

    loop {
        // Очищаем буфер перед чтением новой строки.
        line.clear();

        // Читаем строку вместе с символом '\n'.
        let bytes_read = reader.read_line(&mut line).expect("Cannot read line");

        // 0 байтов означает конец файла.
        if bytes_read == 0 {
            break;
        }

        total_lines += 1;

        // Проверяем наличие подстроки без учёта регистра.
        if contains_text(&line, search_text) {
            // Сохраняем исходную строку без изменений.
            // Перенос строки уже содержится в line.
            output
                .write_all(line.as_bytes())
                .expect("Cannot write matching line");

            matching_lines += 1;
        }
    }

    // Завершаем запись.
    output.flush().expect("Cannot flush output file");

    println!("Input file: {}", input_filename);
    println!("Output file: {}", output_filename);
    println!("Search text: {}", search_text);
    println!("Lines processed: {}", total_lines);
    println!("Matching lines: {}", matching_lines);
}

// --------------------------------------------------
// C) CREATE SAMPLE FILE
// --------------------------------------------------

// Создаём демонстрационный файл, если его нет.
//
// Если aaa.txt уже существует, не изменяем его.
fn create_sample_file(filename: &str) {
    if Path::new(filename).exists() {
        return;
    }

    let mut file = File::create(filename).expect("Cannot create sample file");

    // Данные для проверки из условия преподавателя.
    writeln!(file, "Dnes je sobota.").expect("Cannot write sample");

    writeln!(file, "Zitra je nedele.").expect("Cannot write sample");

    writeln!(file, "vcera, dnes, zitra").expect("Cannot write sample");

    writeln!(file, "Tohle je ukazkovy text.").expect("Cannot write sample");

    writeln!(file, "dnesni").expect("Cannot write sample");

    writeln!(file, "DNES BUDE HEZKY DEN.").expect("Cannot write sample");
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    let input_filename = "aaa.txt";

    // Отдельное имя, чтобы не перезаписать novy.txt
    // из задания 08_04.
    let output_filename = "novy_08_05.txt";

    let search_text = "dnes";

    println!("TEXT FILE LINE FILTER");
    println!("========================================");

    // Создаём пример, только если входного файла нет.
    create_sample_file(input_filename);

    // Выполняем поиск.
    filter_lines(input_filename, output_filename, search_text);

    println!("Filtering completed successfully.");

    // Показываем результат.
    println!();
    println!("MATCHING LINES");
    println!("----------------------------------------");

    let output = std::fs::read_to_string(output_filename).expect("Cannot read output file");

    print!("{}", output);
}
