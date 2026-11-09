/*
Cvičení 08 — Úloha 8.6
Název: Statistické údaje
Body: 1 bod; 2 body; 2 body; 2 body; 2 body (v pořadí podúloh)

Zadání:
Napište funkce, které ze zadaného textového souboru zjistí následující statistické údaje.

a) Funkce vracející počet různých znaků, které se v souboru vyskytují.  [1 bod]

b) Funkce vracející počet slov, které se v souboru vyskytují. Definice slova je uvedena v příkladu
6.1. [2 body]

c) Funkce vracející počet vět, které se v souboru vyskytují. Věta obsahuje alespoň jedno slovo a je
ukončena tečkou, otazníkem nebo vykřičníkem. Věta nemusí začínat velkým písmenem. Pokud první
písmeno, které následuje po tečce je malé, tato tečka neukončuje větu. Věta se může nacházet na
více řádcích. [2 body]

d) Funkce vracející nejdelší slovo, vyskytující se v souboru. V případě, že dvě slova mají shodný
počet písmen, vypíše se první z nich. [2 body]

e) Funkce vracející nejdelší větu, vyskytující se v souboru. Délka vět je měřena počtem všech
znaků, které obsahuje. V případě, že dvě věty mají shodný počet znaků, vypíše se první z nich. [2
body]


Ukázka:

int pismen = PocetRuznychZnaku("aaa.txt"); // 36

int slov = PocetSlov("aaa.txt"); // 26

int vet = PocetVet("aaa.txt"); // 5

string slovo = NejdelsiSlovo("aaa.txt"); // "ukazkovy"

string veta = NejdelsiVeta("aaa.txt");

// "Nadpis textu\nTohle je 4. ukazkovy textovy soubor.\n"

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

use std::fs;

// --------------------------------------------------
// HELPERS
// --------------------------------------------------

// Проверяем, является ли байт английской буквой.
//
// Это определение слова из упражнения 06_01.
fn is_letter(character: u8) -> bool {
    (character >= b'A' && character <= b'Z') || (character >= b'a' && character <= b'z')
}

// Проверяем, является ли символ строчной буквой.
//
// Используем char, чтобы учитывать и чешские буквы.
fn is_lowercase_letter(character: char) -> bool {
    character.is_alphabetic() && character.is_lowercase()
}

// Проверяем, завершает ли знак предложение.
//
// '?' и '!' всегда завершают предложение.
//
// '.' не завершает предложение, если первая
// следующая буква является строчной.
//
// Например:
// "Toto je 4. ukazkovy text."
// содержит одно предложение.
fn ends_sentence(text: &str, position: usize, character: char) -> bool {
    if character == '?' || character == '!' {
        return true;
    }

    if character != '.' {
        return false;
    }

    // Ищем первую букву после точки.
    let remaining = &text[position + character.len_utf8()..];

    let mut i = 0;

    while i < remaining.len() {
        let next = remaining[i..]
            .chars()
            .next()
            .expect("Invalid text position");

        if next.is_alphabetic() {
            // Если буква строчная, точка
            // не завершает предложение.
            if is_lowercase_letter(next) {
                return false;
            }

            return true;
        }

        i += next.len_utf8();
    }

    // Если после точки больше нет букв,
    // считаем её концом предложения.
    true
}

// --------------------------------------------------
// A) COUNT DISTINCT CHARACTERS
// --------------------------------------------------

// Считаем количество различных символов.
//
// Например:
// "abac" -> 3 (a, b, c).
//
// Учитываем все символы:
// буквы, цифры, пробелы, знаки препинания
// и переносы строк.
//
// Не используем HashSet или другие коллекции.
fn count_distinct_characters(text: &str) -> usize {
    let mut count = 0;

    // Текущая позиция в байтах UTF-8.
    let mut i = 0;

    while i < text.len() {
        let current = text[i..]
            .chars()
            .next()
            .expect("Invalid character position");

        // Проверяем, встречался ли этот символ раньше.
        let mut found = false;
        let mut j = 0;

        while j < i {
            let previous = text[j..]
                .chars()
                .next()
                .expect("Invalid character position");

            if current == previous {
                found = true;
                break;
            }

            j += previous.len_utf8();
        }

        // Если раньше такого символа не было,
        // увеличиваем количество.
        if found == false {
            count += 1;
        }

        i += current.len_utf8();
    }

    count
}

// --------------------------------------------------
// B) COUNT WORDS
// --------------------------------------------------

// Считаем слова во всём файле.
//
// Слово — последовательность английских букв.
//
// Например:
// "Alena ma 2 ruce." -> 3 слова.
//
// Цифры не считаются буквами.
fn count_words(text: &str) -> usize {
    let bytes = text.as_bytes();

    let mut count = 0;
    let mut inside_word = false;
    let mut i = 0;

    while i < bytes.len() {
        if is_letter(bytes[i]) {
            // Начало нового слова.
            if inside_word == false {
                count += 1;
                inside_word = true;
            }
        } else {
            // Текущее слово закончилось.
            inside_word = false;
        }

        i += 1;
    }

    count
}

// --------------------------------------------------
// C) COUNT SENTENCES
// --------------------------------------------------

// Считаем завершённые предложения.
//
// Предложение должно содержать хотя бы одно слово
// и завершаться '.', '?' или '!'.
//
// Предложение может занимать несколько строк.
fn count_sentences(text: &str) -> usize {
    let mut count = 0;
    let mut has_word = false;

    let mut i = 0;

    while i < text.len() {
        let character = text[i..]
            .chars()
            .next()
            .expect("Invalid character position");

        // Определяем наличие слова.
        if character.is_alphabetic() {
            has_word = true;
        }

        // Проверяем конец предложения.
        if has_word && ends_sentence(text, i, character) {
            count += 1;
            has_word = false;
        }

        i += character.len_utf8();
    }

    count
}

// --------------------------------------------------
// D) LONGEST WORD
// --------------------------------------------------

// Находим самое длинное слово.
//
// Если несколько слов имеют одинаковую длину,
// возвращаем первое.
//
// Возвращаем срез исходного текста.
// Поэтому не нужно копировать слово в новую строку.
fn longest_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    let mut best_start = 0;
    let mut best_length = 0;

    let mut i = 0;

    while i < bytes.len() {
        // Пропускаем символы, которые не являются буквами.
        while i < bytes.len() && is_letter(bytes[i]) == false {
            i += 1;
        }

        // Запоминаем начало слова.
        let start = i;

        // Проходим все буквы текущего слова.
        while i < bytes.len() && is_letter(bytes[i]) {
            i += 1;
        }

        let length = i - start;

        // Используем строго >, а не >=.
        // Поэтому при равной длине останется первое слово.
        if length > best_length {
            best_length = length;
            best_start = start;
        }
    }

    &text[best_start..best_start + best_length]
}

// --------------------------------------------------
// E) LONGEST SENTENCE
// --------------------------------------------------

// Находим самое длинное завершённое предложение.
//
// Длину измеряем количеством символов,
// а не количеством байтов.
//
// Сохраняем пробелы и переносы строк.
//
// Если несколько предложений одинаковой длины,
// выбираем первое.
fn longest_sentence(text: &str) -> &str {
    let mut best_start = 0;
    let mut best_end = 0;
    let mut best_length = 0;

    // Начало текущего предложения.
    let mut sentence_start = 0;

    // Содержит ли текущее предложение хотя бы слово.
    let mut has_word = false;

    let mut i = 0;

    while i < text.len() {
        let character = text[i..]
            .chars()
            .next()
            .expect("Invalid character position");

        if character.is_alphabetic() {
            has_word = true;
        }

        if has_word && ends_sentence(text, i, character) {
            // Конец предложения после знака пунктуации.
            let mut end = i + character.len_utf8();

            // Включаем пробелы и переносы строк
            // после знака конца предложения.
            //
            // Это позволяет сохранить форматирование
            // исходного файла.
            while end < text.len() {
                let next = text[end..]
                    .chars()
                    .next()
                    .expect("Invalid character position");

                if next.is_whitespace() == false {
                    break;
                }

                end += next.len_utf8();
            }

            // Длина предложения в Unicode-символах.
            let length = text[sentence_start..end].chars().count();

            // Сохраняем только более длинное предложение.
            if length > best_length {
                best_length = length;
                best_start = sentence_start;
                best_end = end;
            }

            // Следующее предложение начинается
            // после текущего.
            sentence_start = end;
            has_word = false;
            i = end;

            continue;
        }

        i += character.len_utf8();
    }

    // Если завершённых предложений нет,
    // возвращаем пустую строку.
    &text[best_start..best_end]
}

// --------------------------------------------------
// FILE STATISTICS
// --------------------------------------------------

// Открываем текстовый файл и вычисляем статистику.
//
// Файл читается только один раз.
// Затем все функции работают с его содержимым.
fn print_file_statistics(filename: &str) {
    // Читаем весь файл в String.
    //
    // read_to_string() ожидает корректный UTF-8.
    let text = fs::read_to_string(filename).expect("Cannot read input file");

    // A) Различные символы.
    let characters = count_distinct_characters(&text);

    // B) Количество слов.
    let words = count_words(&text);

    // C) Количество предложений.
    let sentences = count_sentences(&text);

    // D) Самое длинное слово.
    let word = longest_word(&text);

    // E) Самое длинное предложение.
    let sentence = longest_sentence(&text);

    println!("Input file: {}", filename);
    println!();

    println!("A) Distinct characters: {}", characters);
    println!("B) Words: {}", words);
    println!("C) Sentences: {}", sentences);
    println!("D) Longest word: {}", word);

    println!("E) Longest sentence:");
    println!("----------------------------------------");

    // {:?} показывает переносы строк как \n,
    // поэтому удобно проверять точные границы.
    println!("{:?}", sentence);

    println!("----------------------------------------");
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    let filename = "aaa.txt";

    println!("TEXT FILE STATISTICS");
    println!("========================================");

    // Обрабатываем файл из задания.
    print_file_statistics(filename);

    println!();
    println!("Statistics calculated successfully.");
}
