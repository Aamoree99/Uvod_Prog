/*
Cvičení 08 — Úloha 8.2
Název: Převod binárního souboru na textový
Body: 3 body

Zadání:
Napište funkci, která ze zadaného binárního souboru obsahujícího kladné celá čísla vytvoří textový
soubor, který bude na každém řádku obsahovat 10 čísel.  [3 body].


Ukázka:

BinarniNaTextovy("cisla.dat", "cisla.txt");

/* 1540684667 1050586463 ... 487858389  // 10 čísel

 *  2103328287 817864052  ... 727647820  // 10 čísel

 *  ...                                  // 10 čísel

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
use std::io::{Read, Write};

// --------------------------------------------------
// A) BINARY TO TEXT
// --------------------------------------------------

// Преобразуем бинарный файл в текстовый.
//
// input_filename  — исходный бинарный файл.
// output_filename — итоговый текстовый файл.
//
// Каждое число занимает 4 байта (i32).
// В текстовом файле записываем по 10 чисел в строке.
fn binary_to_text(input_filename: &str, output_filename: &str) {
    // Открываем существующий бинарный файл.
    let mut input = File::open(input_filename).expect("Cannot open binary file");

    // Создаём текстовый файл.
    // Если он уже существует, его содержимое перезапишется.
    let mut output = File::create(output_filename).expect("Cannot create text file");

    // Проверяем размер исходного файла.
    let file_size = input.metadata().expect("Cannot read file metadata").len();

    // Одно число занимает ровно 4 байта.
    assert!(file_size % 4 == 0, "Invalid binary file size");

    let total_numbers = file_size / 4;

    // Буфер для одного числа из 4 байтов.
    let mut bytes = [0u8; 4];

    // Счётчик чисел в текущей строке.
    let mut numbers_in_line = 0;

    // Счётчик всех прочитанных чисел.
    let mut count = 0;

    while count < total_numbers {
        // Читаем следующие 4 байта.
        //
        // read_exact() гарантирует, что будут прочитаны
        // все 4 байта, иначе вернёт ошибку.
        input.read_exact(&mut bytes).expect("Cannot read integer");

        // Преобразуем 4 байта обратно в i32.
        //
        // Используем from_le_bytes(), потому что
        // в 08_01 записывали через to_le_bytes().
        let number = i32::from_le_bytes(bytes);

        // Добавляем пробел между числами одной строки.
        if numbers_in_line > 0 {
            write!(output, " ").expect("Cannot write separator");
        }

        // Записываем число как обычный текст.
        write!(output, "{}", number).expect("Cannot write number");

        numbers_in_line += 1;
        count += 1;

        // После каждых 10 чисел переходим
        // на следующую строку.
        if numbers_in_line == 10 {
            writeln!(output).expect("Cannot write newline");

            numbers_in_line = 0;
        }
    }

    // Если последняя строка неполная,
    // всё равно завершаем её переводом строки.
    if numbers_in_line > 0 {
        writeln!(output).expect("Cannot write final newline");
    }

    // Завершаем запись файла.
    output.flush().expect("Cannot flush text file");

    println!("Binary file: {}", input_filename);
    println!("Text file: {}", output_filename);
    println!("Numbers converted: {}", count);
    println!("Numbers per line: 10");

    // Для 1000 чисел ожидаем ровно 100 строк.
    let lines = (count + 9) / 10;

    println!("Lines written: {}", lines);
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    let input_filename = "cisla.dat";
    let output_filename = "cisla.txt";

    println!("BINARY TO TEXT CONVERTER");
    println!("========================================");

    // Преобразуем файл из предыдущего задания.
    binary_to_text(input_filename, output_filename);

    println!("Conversion completed successfully.");
}
