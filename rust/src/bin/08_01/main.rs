/*
Cvičení 08 — Úloha 8.1
Název: Náhodný binární soubor
Body: 2 body

Zadání:
Napište funkci, která vytvoří binární soubor obsahující zadaný počet náhodných kladných celých
čísel. [2 body].


Ukázka:

NahodnySoubor("cisla.dat", 1000);

// velikost souboru 4000 bytu

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

use rand::Rng;
use std::fs::File;
use std::io::Write;

// --------------------------------------------------
// A) RANDOM BINARY FILE
// --------------------------------------------------

// Создаём бинарный файл с заданным количеством
// случайных положительных целых чисел.
//
// filename — имя создаваемого файла.
// count — количество чисел.
//
// Каждое число сохраняется как i32 (4 байта).
fn create_random_file(filename: &str, count: usize) {
    // Создаём файл.
    // Если файл существует, его содержимое перезаписывается.
    let mut file = File::create(filename).expect("Cannot create file");

    // Генератор случайных чисел.
    let mut rng = rand::thread_rng();

    let mut i = 0;

    while i < count {
        // Генерируем положительное целое число.
        // Диапазон: от 1 до максимального i32.
        let number: i32 = rng.gen_range(1..=i32::MAX);

        // Преобразуем число в массив из 4 байтов.
        //
        // to_le_bytes() использует Little Endian:
        // младший байт записывается первым.
        let bytes = number.to_le_bytes();

        // Записываем непосредственно 4 байта в файл,
        // а не текстовое представление числа.
        file.write_all(&bytes).expect("Cannot write to file");

        i += 1;
    }

    // Завершаем запись и сбрасываем буфер.
    file.flush().expect("Cannot flush file");
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    let filename = "cisla.dat";
    let count = 1000;

    println!("RANDOM BINARY FILE");
    println!("========================================");

    // Создаём файл с 1000 случайных чисел.
    create_random_file(filename, count);

    // Получаем информацию о созданном файле.
    let metadata = std::fs::metadata(filename).expect("Cannot read file metadata");

    let expected_size = count as u64 * 4;
    let actual_size = metadata.len();

    println!("File: {}", filename);
    println!("Numbers: {}", count);
    println!("Integer type: i32");
    println!("Bytes per number: 4");
    println!("Expected size: {} bytes", expected_size);
    println!("Actual size: {} bytes", actual_size);

    // Проверяем, соответствует ли размер условию.
    assert!(actual_size == expected_size, "Incorrect file size");

    println!("File created successfully.");
}
