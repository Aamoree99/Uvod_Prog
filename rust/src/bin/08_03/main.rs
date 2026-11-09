/*
Cvičení 08 — Úloha 8.3
Název: Součet čísel na řádku
Body: 3 body

Zadání:
Napište funkci, která ze zadaného textového souboru obsahujícího celá čísla vytvoří nový textový
soubor, který bude na každém řádku obsahovat součet čísel z odpovídajícího řádku původního souboru.
  [3 body]


Ukázka:

SoucetCisel("cisla.txt", "soucet.txt");

/* 9690170347

 * 12228343489

 * ...

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
// A) SUM NUMBERS IN A LINE
// --------------------------------------------------

// Получаем сумму всех целых чисел в одной строке.
//
// Например:
// "100 200 300" -> 600
//
// Числа разделены пробелами или табуляцией.
// Поддерживаются и отрицательные числа.
//
// Не используем split() или коллекции.
fn sum_line(line: &str) -> i64 {
    let bytes = line.as_bytes();

    let mut sum: i64 = 0;
    let mut i = 0;

    // Проходим строку посимвольно.
    while i < bytes.len() {
        // Пропускаем пробелы и табуляцию.
        while i < bytes.len()
            && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n' || bytes[i] == b'\r')
        {
            i += 1;
        }

        // Если дошли до конца строки, выходим.
        if i >= bytes.len() {
            break;
        }

        // Запоминаем знак текущего числа.
        let mut negative = false;

        if bytes[i] == b'-' {
            negative = true;
            i += 1;
        } else if bytes[i] == b'+' {
            i += 1;
        }

        // Собираем число из отдельных цифр.
        let mut number: i64 = 0;
        let mut has_digits = false;

        while i < bytes.len() && bytes[i] >= b'0' && bytes[i] <= b'9' {
            let digit = (bytes[i] - b'0') as i64;

            // Например, число 123:
            // 0 * 10 + 1 = 1
            // 1 * 10 + 2 = 12
            // 12 * 10 + 3 = 123
            number = number
                .checked_mul(10)
                .and_then(|value| value.checked_add(digit))
                .expect("Integer overflow");

            has_digits = true;
            i += 1;
        }

        assert!(has_digits, "Invalid number in input file");

        // Применяем знак.
        if negative {
            number = -number;
        }

        // Добавляем число к сумме строки.
        sum = sum.checked_add(number).expect("Sum overflow");

        // После числа должен быть разделитель
        // или конец строки.
        if i < bytes.len() {
            assert!(
                bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n' || bytes[i] == b'\r',
                "Invalid character in input file"
            );
        }
    }

    sum
}

// --------------------------------------------------
// B) SUM NUMBERS FROM FILE
// --------------------------------------------------

// Читаем исходный файл построчно.
//
// Для каждой строки:
// 1. Считываем текст.
// 2. Вычисляем сумму чисел.
// 3. Записываем сумму в выходной файл.
//
// Каждая исходная строка соответствует
// одной строке выходного файла.
fn sum_numbers_in_file(input_filename: &str, output_filename: &str) {
    // Открываем исходный текстовый файл.
    let input = File::open(input_filename).expect("Cannot open input file");

    // BufReader позволяет читать файл построчно.
    let mut reader = BufReader::new(input);

    // Создаём новый файл для результатов.
    let mut output = File::create(output_filename).expect("Cannot create output file");

    // Буфер для текущей строки.
    let mut line = String::new();

    // Количество обработанных строк.
    let mut line_count = 0;

    loop {
        // Очищаем строку перед следующим чтением.
        line.clear();

        // Считываем одну строку, включая '\n'.
        //
        // Функция возвращает количество прочитанных байтов.
        // Если прочитано 0 байтов, файл закончился.
        let bytes_read = reader.read_line(&mut line).expect("Cannot read line");

        if bytes_read == 0 {
            break;
        }

        // Вычисляем сумму чисел в строке.
        let sum = sum_line(&line);

        // Записываем сумму на отдельную строку.
        writeln!(output, "{}", sum).expect("Cannot write sum");

        line_count += 1;
    }

    // Сбрасываем буфер записи.
    output.flush().expect("Cannot flush output file");

    println!("Input file: {}", input_filename);
    println!("Output file: {}", output_filename);
    println!("Lines processed: {}", line_count);
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    let input_filename = "cisla.txt";
    let output_filename = "soucet.txt";

    println!("SUM NUMBERS IN EACH LINE");
    println!("========================================");

    // Обрабатываем файл из задания 08_02.
    sum_numbers_in_file(input_filename, output_filename);

    println!("File processed successfully.");
}
