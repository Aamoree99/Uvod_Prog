/*
Cvičení 08 — Úloha 8.4
Název: Změna velikosti písmen
Body: 3 body

Zadání:
Napište funkci, která ze zadaného textového souboru vytvoří nový textový soubor, který se bude od
původního souboru lišit jen ve velikosti písmen. V novém souboru bude první řádek velkými písmeny.
Na ostatních řádcích budou všechna slova obsahující tři a více písmen začínat velkými písmeny.
Zbylá písmena v novém souboru budou malá. [3 body].


Ukázka:

ZmenaVelikostiPismen("aaa.txt", "novy.txt");

/* NADPIS TEXTU

 * Tohle je Ukazkovy Textovy Soubor, na Kterem Budeme Testovat

 * Druhy Radek Cvicneho Textu.

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

// --------------------------------------------------
// A) CHANGE LETTER CASE
// --------------------------------------------------

// Обрабатываем одну строку текста.
//
// first_line = true:
// все буквы переводим в верхний регистр.
//
// first_line = false:
// слова длиной от 3 букв начинаем с заглавной,
// остальные буквы делаем строчными.
//
// Знаки препинания, пробелы и другие символы
// сохраняем без изменений.
fn change_line(line: &str, first_line: bool) -> String {
    let mut result = String::new();

    // Первую строку целиком переводим
    // в верхний регистр.
    if first_line {
        for character in line.chars() {
            // Некоторые Unicode-буквы при преобразовании
            // могут превращаться в несколько символов,
            // поэтому используем extend().
            result.extend(character.to_uppercase());
        }

        return result;
    }

    // Начальная позиция текущего слова в байтах.
    let mut word_start = 0;

    // Позиция при поиске слов.
    let mut position = 0;

    // Количество букв в текущем слове.
    let mut word_length = 0;

    // Проходим по Unicode-символам.
    for character in line.chars() {
        if character.is_alphabetic() {
            // Начало нового слова.
            if word_length == 0 {
                word_start = position;
            }

            word_length += 1;
        } else {
            // Если закончилось слово, обрабатываем его.
            if word_length > 0 {
                let word = &line[word_start..position];

                // Слова длиной от 3 букв:
                // первая буква заглавная,
                // остальные строчные.
                if word_length >= 3 {
                    let mut first_character = true;

                    for letter in word.chars() {
                        if first_character {
                            result.extend(letter.to_uppercase());
                            first_character = false;
                        } else {
                            result.extend(letter.to_lowercase());
                        }
                    }
                } else {
                    // Короткие слова полностью строчные.
                    for letter in word.chars() {
                        result.extend(letter.to_lowercase());
                    }
                }

                word_length = 0;
            }

            // Символ-разделитель сохраняем.
            result.push(character);
        }

        // len_utf8() возвращает размер символа в байтах.
        // Это необходимо для правильного среза строки.
        position += character.len_utf8();
    }

    // Если строка закончилась прямо после слова,
    // его тоже нужно обработать.
    if word_length > 0 {
        let word = &line[word_start..position];

        if word_length >= 3 {
            let mut first_character = true;

            for letter in word.chars() {
                if first_character {
                    result.extend(letter.to_uppercase());
                    first_character = false;
                } else {
                    result.extend(letter.to_lowercase());
                }
            }
        } else {
            for letter in word.chars() {
                result.extend(letter.to_lowercase());
            }
        }
    }

    result
}

// --------------------------------------------------
// B) PROCESS TEXT FILE
// --------------------------------------------------

// Читаем исходный файл построчно,
// преобразуем регистр букв и записываем
// результат в новый текстовый файл.
fn change_letter_case(input_filename: &str, output_filename: &str) {
    // Открываем исходный текстовый файл.
    let input = File::open(input_filename).expect("Cannot open input file");

    // Буферизованное чтение строк.
    let mut reader = BufReader::new(input);

    // Создаём файл для результата.
    // Существующий файл будет перезаписан.
    let mut output = File::create(output_filename).expect("Cannot create output file");

    // Буфер для одной строки.
    let mut line = String::new();

    // Номер текущей строки.
    let mut line_count = 0;

    loop {
        // Очищаем буфер перед чтением.
        line.clear();

        // read_line() сохраняет символы переноса строки.
        let bytes_read = reader.read_line(&mut line).expect("Cannot read line");

        // Конец файла.
        if bytes_read == 0 {
            break;
        }

        // Первая строка обрабатывается отдельно.
        let first_line = line_count == 0;

        // Преобразуем регистр.
        let changed = change_line(&line, first_line);

        // Используем write!, а не writeln!,
        // поскольку перенос строки уже есть в changed.
        output
            .write_all(changed.as_bytes())
            .expect("Cannot write text");

        line_count += 1;
    }

    output.flush().expect("Cannot flush output file");

    println!("Input file: {}", input_filename);
    println!("Output file: {}", output_filename);
    println!("Lines processed: {}", line_count);
}

// --------------------------------------------------
// C) CREATE SAMPLE FILE
// --------------------------------------------------

// Создаём пример исходного файла,
// если он ещё не существует.
//
// Это позволяет запускать 08_04 независимо
// от остальных упражнений.
fn create_sample_file(filename: &str) {
    // Если файл уже есть, ничего не меняем.
    if std::path::Path::new(filename).exists() {
        return;
    }

    let mut file = File::create(filename).expect("Cannot create sample file");

    // Текст из примера задания,
    // специально с разным регистром.
    writeln!(file, "nadpis textu").expect("Cannot write sample");

    writeln!(
        file,
        "tOHLE JE uKAZKOVY tEXTOVY sOUBOR, NA kTEREM bUDEME tESTOVAT"
    )
    .expect("Cannot write sample");

    writeln!(file, "DRUHY RADEK CVICNEHO TEXTU.").expect("Cannot write sample");

    // Проверка чешских букв.
    writeln!(file, "pŘÍLIŠ ŽLUŤOUČKÝ kŮŇ a čaj.").expect("Cannot write sample");
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    let input_filename = "aaa.txt";
    let output_filename = "novy.txt";

    println!("LETTER CASE CONVERTER");
    println!("========================================");

    // Если входного файла нет, создаём
    // демонстрационный текст.
    create_sample_file(input_filename);

    // Выполняем преобразование.
    change_letter_case(input_filename, output_filename);

    println!("Conversion completed successfully.");

    // Показываем готовый результат.
    println!();
    println!("OUTPUT FILE CONTENT:");
    println!("----------------------------------------");

    let output = std::fs::read_to_string(output_filename).expect("Cannot read output file");

    print!("{}", output);
}
