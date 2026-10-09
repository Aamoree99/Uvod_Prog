/*
Cvičení 07 — Úloha 7.4
Název: Římská čísla
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Římská čísla jsou příkladem nepoziční číselné soustavy. Hodnoty číslic nezávisí na jejich pozici
v zápise čísla. Hodnoty číslic jsou následující I = 1, V = 5, X = 10, L = 50, C = 100, D = 500, M =
1000. Existují i dvojice číslic, které se považují za jednu číslici: IV  = 4, IX = 9, XL = 40, XC =
90, CD 400, CM 900. Číslice jsou v zápisu čísla uspořádány od nejvyšší hodnoty po nejnižší.
Například číslo 2016 se zapíše jako MMXVI. Například číslo 1949 se zapíše zkráceným zápisem jako
MCMXLIX. Například výraz IM, který měl mít hodnotu 999, odporuje výše uvedeným pravidlům a není
povolen. Napište následující funkce:

a) Funkce DecRoman převede číslo v desítkové soustavě na římské číslo. [1 bod]

b) Funkce RomanDec převede římské číslo na číslo v desítkové soustavě [1 bod].


Ukázka:

string s = DecRoman(1969); // "MCMLXIX"

int i = RomanDec("MMCDXXXVII"); // 2437


Správnost programu si můžete ověřit na online konvertoru:

http://www.convertit.com/Go/ConvertIt/Calculators/Math/Roman_Numerals_Converter.ASP

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
// --------------------------------------------------
// A) DECIMAL TO ROMAN
// --------------------------------------------------

// Переводим обычное число в римское.
//
// Используем массивы значений и соответствующих
// римских обозначений.
//
// Включаем специальные сочетания:
// IV, IX, XL, XC, CD, CM.
//
// Например:
// 1969 -> MCMLXIX
fn decimal_to_roman(number: i32) -> String {
    // Стандартная римская запись поддерживает 1–3999.
    assert!(
        number >= 1 && number <= 3999,
        "Number must be between 1 and 3999"
    );

    // Значения расположены по убыванию.
    let values: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];

    // Соответствующие римские символы.
    let symbols: [&str; 13] = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];

    // Строка для результата.
    let mut result = String::new();

    // Оставшаяся часть числа.
    let mut remaining = number;

    let mut i = 0;

    // Перебираем значения от самого большого.
    while i < values.len() {
        // Пока значение помещается в оставшееся число,
        // добавляем его римское обозначение.
        while remaining >= values[i] {
            result.push_str(symbols[i]);

            // Вычитаем использованное значение.
            remaining -= values[i];
        }

        i += 1;
    }

    result
}

// --------------------------------------------------
// HELPER: ROMAN SYMBOL VALUE
// --------------------------------------------------

// Возвращаем числовое значение римского символа.
//
// Если символ неизвестен, возвращаем 0.
fn roman_value(character: u8) -> i32 {
    match character {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        b'M' => 1000,
        _ => 0,
    }
}

// --------------------------------------------------
// B) ROMAN TO DECIMAL
// --------------------------------------------------

// Переводим римское число в десятичное.
//
// Если следующий символ больше текущего,
// текущий символ вычитается.
//
// Например:
// IV = -1 + 5 = 4
// IX = -1 + 10 = 9
// XL = -10 + 50 = 40
//
// Проверяем правильность записи, преобразуя
// полученное число обратно в римскую форму.
//
// При ошибке возвращаем -1.
fn roman_to_decimal(text: &str) -> i32 {
    let bytes = text.as_bytes();

    // Пустая строка недопустима.
    if bytes.len() == 0 {
        return -1;
    }

    let mut result = 0;
    let mut i = 0;

    while i < bytes.len() {
        // Значение текущего символа.
        let current = roman_value(bytes[i]);

        // Неизвестный символ — ошибка.
        if current == 0 {
            return -1;
        }

        // Проверяем следующий символ.
        if i + 1 < bytes.len() {
            let next = roman_value(bytes[i + 1]);

            if next == 0 {
                return -1;
            }

            if current < next {
                // Если следующий символ больше,
                // вычитаем текущий.
                result -= current;
            } else {
                // Иначе прибавляем.
                result += current;
            }
        } else {
            // Последний символ всегда прибавляем.
            result += current;
        }

        i += 1;
    }

    // Проверяем допустимый диапазон.
    if result < 1 || result > 3999 {
        return -1;
    }

    // Преобразуем полученное число обратно
    // в каноническую римскую запись.
    //
    // Например, IM даст 999,
    // но правильная запись 999 = CMXCIX.
    //
    // Поэтому IM будет отклонено.
    let correct_roman = decimal_to_roman(result);

    if correct_roman != text {
        return -1;
    }

    result
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    println!("ROMAN NUMERAL CONVERTER");
    println!("========================================");

    // A) Пример из задания.
    let decimal = 1969;

    let roman = decimal_to_roman(decimal);

    println!("A) DECIMAL TO ROMAN");
    println!("Decimal: {}", decimal);
    println!("Roman:   {}", roman);

    // B) Пример из задания.
    let roman_text = "MMCDXXXVII";

    let decimal_result = roman_to_decimal(roman_text);

    println!();
    println!("B) ROMAN TO DECIMAL");
    println!("Roman:   {}", roman_text);
    println!("Decimal: {}", decimal_result);

    // Дополнительные проверки правильных чисел.
    println!();
    println!("ADDITIONAL TESTS");

    println!("4    -> {}", decimal_to_roman(4));
    println!("9    -> {}", decimal_to_roman(9));
    println!("49   -> {}", decimal_to_roman(49));
    println!("999  -> {}", decimal_to_roman(999));
    println!("2016 -> {}", decimal_to_roman(2016));
    println!("3999 -> {}", decimal_to_roman(3999));

    // Дополнительные проверки неправильных записей.
    println!();
    println!("INVALID ROMAN NUMERALS");

    println!("IM   -> {}", roman_to_decimal("IM"));
    println!("IIII -> {}", roman_to_decimal("IIII"));
    println!("VX   -> {}", roman_to_decimal("VX"));
    println!("IC   -> {}", roman_to_decimal("IC"));
    println!("ABC  -> {}", roman_to_decimal("ABC"));
    println!("     -> {}", roman_to_decimal(""));

    // Проверяем, что перевод в обе стороны
    // сохраняет исходное число.
    println!();
    println!("ROUND-TRIP TEST");

    let mut number = 1;
    let mut correct = true;

    // Проверяем все числа от 1 до 3999.
    while number <= 3999 {
        let roman = decimal_to_roman(number);
        let restored = roman_to_decimal(&roman);

        if restored != number {
            correct = false;
            break;
        }

        number += 1;
    }

    println!("All numbers 1-3999 passed: {}", correct);
}
