/*
Cvičení 08 — Úloha 8.8
Název: Formátování textu
Body: 2 body; 2 bod; 2 body (v pořadí podúloh)

Zadání:
Napište funkci, která ze zadaného textového souboru vytvoří druhý textový soubor, ve kterém bude
text zalomen na zadanou délku řádky (obdoba funkcí z MS Word). Řádek bude naplněn slovy tak, že
nepůjde na něj žádné další slovo přidat bez porušení požadované délky řádku. Za slovo se (na rozdíl
od předchozích příkladů) považuje jakýkoliv řetězec z jiných než bílých znaků. Slova mohou být
kombinací písmen, čísel, interpunkčních znamének atd. Bílé znaky a řídící znaky jsou odstraněny a
nahrazeny jednou mezerou mezi každými sousedními řetězci. Slovo nesmí být rozděleno na více řádků
s výjimkou případu, kdy je delší než maximální délka řádku. Zbylé místo na řádku je vyplněno
mezerami. Vytvořte výčtový typ Format, který bude určovat, jakým způsobem se umísťují výplňkové
mezery na řádce.

a) Text se bude formátovat směrem doleva. Všechny výplňkové mezery jsou přidány za poslední slovo.
[2 body]

b) Text se bude formátovat směrem doprava. Všechny výplňkové mezery jsou přidány před první slovo.
[2 bod]

c) Text se bude formátovat na střed. Všechny výplňkové mezery jsou rozděleny na dvě poloviny.
V případě lichého počtu mezer bude druhá polovina o mezeru větší. První polovina je přidána před
první slovo, druhá polovina za poslední slovo [2 body]


Ukázka:

VypisFormat("aaa.txt", "doleva.txt", Format.DoLeva, 20);

/* Nadpis textu Tohle

 * je 5. ukazkovy

 * textovy soubor.

 */

VypisFormat("aaa.txt", "doprava.txt", Format.DoPrava, 20);

/*  Nadpis textu Tohle

 *      je 5. ukazkovy

 *     textovy soubor.

 */

VypisFormat("aaa.txt", "nastred.txt", Format.NaStred, 20);

/* Nadpis textu Tohle

 *   je 5. ukazkovy

 *  textovy soubor.

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

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// --------------------------------------------------
// FORMAT TYPE
// --------------------------------------------------

// Тип перечисления, требуемый преподавателем.
//
// Left   — выравнивание влево.
// Right  — выравнивание вправо.
// Center — выравнивание по центру.
#[derive(Clone, Copy)]
enum Format {
    Left,
    Right,
    Center,
}

// --------------------------------------------------
// A) READ NEXT WORD
// --------------------------------------------------

// Возвращаем следующее слово из текста.
//
// Словом считаем любую последовательность
// символов, кроме пробельных и управляющих.
//
// position — текущая позиция в байтах UTF-8.
//
// Если слова закончились, возвращаем None.
//
// Срез &str не копирует текст в новую строку.
fn next_word<'a>(text: &'a str, position: &mut usize) -> Option<&'a str> {
    // Пропускаем пробелы, табуляции, переносы строк
    // и другие управляющие символы.
    while *position < text.len() {
        let character = text[*position..]
            .chars()
            .next()
            .expect("Invalid character position");

        if character.is_whitespace() || character.is_control() {
            *position += character.len_utf8();
        } else {
            break;
        }
    }

    // Если дошли до конца текста, слов больше нет.
    if *position >= text.len() {
        return None;
    }

    // Запоминаем начало слова.
    let start = *position;

    // Ищем конец слова.
    while *position < text.len() {
        let character = text[*position..]
            .chars()
            .next()
            .expect("Invalid character position");

        if character.is_whitespace() || character.is_control() {
            break;
        }

        *position += character.len_utf8();
    }

    // Возвращаем найденный фрагмент.
    Some(&text[start..*position])
}

// --------------------------------------------------
// B) WRITE SPACES
// --------------------------------------------------

// Записываем заданное количество пробелов.
//
// Не используем коллекции или repeat().
fn write_spaces(output: &mut File, count: usize) {
    let mut i = 0;

    while i < count {
        write!(output, " ").expect("Cannot write space");

        i += 1;
    }
}

// --------------------------------------------------
// C) WRITE FORMATTED LINE
// --------------------------------------------------

// Записываем строку с нужным выравниванием.
//
// line_length — количество Unicode-символов,
// а не байтов.
//
// Все строки дополняются пробелами до width.
fn write_formatted_line(
    output: &mut File,
    line: &str,
    line_length: usize,
    width: usize,
    format: Format,
) {
    assert!(line_length <= width, "Line is too long");

    // Сколько свободных позиций осталось в строке.
    let padding = width - line_length;

    // Количество пробелов слева и справа.
    let left_spaces;
    let right_spaces;

    match format {
        Format::Left => {
            // Все пробелы добавляем справа.
            left_spaces = 0;
            right_spaces = padding;
        }

        Format::Right => {
            // Все пробелы добавляем слева.
            left_spaces = padding;
            right_spaces = 0;
        }

        Format::Center => {
            // Делим свободное место пополам.
            //
            // При нечётном количестве пробелов
            // справа будет на один больше.
            left_spaces = padding / 2;
            right_spaces = padding - left_spaces;
        }
    }

    // Пробелы перед текстом.
    write_spaces(output, left_spaces);

    // Сам текст.
    write!(output, "{}", line).expect("Cannot write line");

    // Пробелы после текста.
    write_spaces(output, right_spaces);

    // Переход на следующую строку.
    writeln!(output).expect("Cannot write newline");
}

// --------------------------------------------------
// D) FORMAT TEXT
// --------------------------------------------------

// Переносим текст на строки заданной ширины.
//
// Каждое слово стараемся разместить целиком.
//
// Если следующее слово уже не помещается,
// записываем текущую строку и начинаем новую.
//
// Если слово длиннее width, разбиваем его
// на части по width символов.
fn format_text(text: &str, output: &mut File, format: Format, width: usize) {
    assert!(width > 0, "Line width must be positive");

    // Текущая строка.
    let mut line = String::new();

    // Длина текущей строки в символах.
    let mut line_length = 0;

    // Текущая позиция в исходном тексте.
    let mut position = 0;

    // Получаем слова по одному.
    while let Some(word) = next_word(text, &mut position) {
        let word_length = word.chars().count();

        // ------------------------------------------
        // CASE 1: WORD LONGER THAN LINE
        // ------------------------------------------

        if word_length > width {
            // Сначала записываем предыдущую строку,
            // если она уже содержит слова.
            if line_length > 0 {
                write_formatted_line(output, &line, line_length, width, format);

                line.clear();
                line_length = 0;
            }

            // Позиция внутри длинного слова.
            let mut word_position = 0;

            while word_position < word.len() {
                // Собираем очередную часть слова.
                let mut part_end = word_position;
                let mut part_length = 0;

                // Берём максимум width символов.
                while part_end < word.len() && part_length < width {
                    let character = word[part_end..]
                        .chars()
                        .next()
                        .expect("Invalid word position");

                    part_end += character.len_utf8();
                    part_length += 1;
                }

                let part = &word[word_position..part_end];

                if part_length == width {
                    // Полная строка из длинного слова.
                    write_formatted_line(output, part, part_length, width, format);
                } else {
                    // Последняя неполная часть слова.
                    // Её можно дополнить следующими словами.
                    line.push_str(part);
                    line_length = part_length;
                }

                word_position = part_end;
            }

            continue;
        }

        // ------------------------------------------
        // CASE 2: FIRST WORD ON LINE
        // ------------------------------------------

        if line_length == 0 {
            line.push_str(word);
            line_length = word_length;

            continue;
        }

        // ------------------------------------------
        // CASE 3: WORD FITS ON CURRENT LINE
        // ------------------------------------------

        // Нужно место для пробела перед словом.
        if line_length + 1 + word_length <= width {
            line.push(' ');
            line.push_str(word);

            line_length += 1 + word_length;
        } else {
            // --------------------------------------
            // CASE 4: WORD DOES NOT FIT
            // --------------------------------------

            // Записываем текущую заполненную строку.
            write_formatted_line(output, &line, line_length, width, format);

            // Начинаем новую строку с этого слова.
            line.clear();
            line.push_str(word);

            line_length = word_length;
        }
    }

    // После обработки всех слов записываем
    // последнюю строку, если она не пустая.
    if line_length > 0 {
        write_formatted_line(output, &line, line_length, width, format);
    }
}

// --------------------------------------------------
// E) FORMAT FILE
// --------------------------------------------------

// Читаем исходный текстовый файл,
// форматируем его и сохраняем результат.
//
// input_filename — исходный файл.
// output_filename — новый файл.
// format — способ выравнивания.
// width — максимальная длина строки.
fn format_file(input_filename: &str, output_filename: &str, format: Format, width: usize) {
    assert!(width > 0, "Line width must be positive");

    // Читаем текст в UTF-8.
    //
    // Здесь исходный файл не изменяется.
    let text = fs::read_to_string(input_filename).expect("Cannot read input file");

    // Создаём выходной файл.
    let mut output = File::create(output_filename).expect("Cannot create output file");

    // Выполняем форматирование.
    format_text(&text, &mut output, format, width);

    // Завершаем запись.
    output.flush().expect("Cannot flush output file");

    println!("Created: {}", output_filename);
}

// --------------------------------------------------
// F) CREATE SAMPLE FILE
// --------------------------------------------------

// Создаём демонстрационный файл,
// если aaa.txt ещё не существует.
//
// Существующий файл НЕ перезаписываем.
fn create_sample_file(filename: &str) {
    if Path::new(filename).exists() {
        return;
    }

    let mut file = File::create(filename).expect("Cannot create sample file");

    // Пример из условия задания.
    writeln!(file, "Nadpis textu Tohle je 5. ukazkovy textovy soubor.")
        .expect("Cannot write sample");
}

// --------------------------------------------------
// G) PRINT RESULT
// --------------------------------------------------

// Показываем содержимое выходного файла.
//
// Для наглядности каждую строку заключаем
// между |, чтобы были видны пробелы.
fn print_result(filename: &str) {
    let text = fs::read_to_string(filename).expect("Cannot read result file");

    println!();
    println!("FILE: {}", filename);

    // Позиция текущей строки.
    let mut start = 0;
    let mut i = 0;

    let bytes = text.as_bytes();

    // Ищем символы конца строки.
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            println!("|{}|", &text[start..i]);

            start = i + 1;
        }

        i += 1;
    }

    // Если последняя строка не заканчивается \n.
    if start < text.len() {
        println!("|{}|", &text[start..]);
    }
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    let input_filename = "aaa.txt";
    let width = 20;

    println!("TEXT FORMATTER");
    println!("========================================");

    // Создаём пример, только если файла ещё нет.
    create_sample_file(input_filename);

    // A) Выравнивание влево.
    format_file(input_filename, "doleva.txt", Format::Left, width);

    // B) Выравнивание вправо.
    format_file(input_filename, "doprava.txt", Format::Right, width);

    // C) Выравнивание по центру.
    format_file(input_filename, "nastred.txt", Format::Center, width);

    println!();
    println!("Formatting completed successfully.");

    // Показываем все три результата.
    print_result("doleva.txt");
    print_result("doprava.txt");
    print_result("nastred.txt");
}
