/*
Cvičení 06 — Úloha 6.1
Název: Práce s textovými řetězci
Body: 2 body; 2 body; 2 body; 1 bod; 1 bod; 4 body (v pořadí podúloh)

Zadání:
Slovo je řetězec složený výhradně z písmen, který je maximální délky, nejde tedy prodloužit o další
písmeno. Dvě slova jsou od sebe oddělena jedním nebo více znaky, které nejsou písmeny. Pokud se
před slovem či za slovem nachází znak, tak tento znak není písmenem. Napište následující funkce pro
práci s textovými řetězci bez použití funkcí Split a Distinct. Původní řetězec nesmíte modifikovat,
ani od něj vytvářet kopie:

a) Funkce vracející počet různých písmen anglické abecedy, které zadaný řetězec obsahuje. Velikost
písmen se nerozlišuje. [2 body]

b) Funkce vracející počet slov v zadaném řetězci. [2 body]

c) Funkce vracející slovo první výskyt slova na zadané pozice v zadaném řetězci. Pokud se na zadané
pozici nachází písmeno, je vráceno slovo, jehož součástí toto písmeno je. V tomto případě bude
nalezené slovo často začínat před zadanou pozicí. Pokud se na zadané pozici nenachází písmeno, je
vráceno první slovo nacházející se napravo od zadané pozice v zadaném řetězci. Pokud není nalezeno
žádné slovo, je vrácen prázdný řetězec. [2 body]

d) Funkce vracející nejdelší slovo v zadaném řetězci. Pokud jsou dvě slova stejně dlouhá, je
vráceno slovo, které se v řetězci vyskytuje jako první. Pokud není nalezeno žádné slovo, je vrácen
prázdný řetězec. [1 bod]

e) Funkce vracející lexikograficky (abecedně) největší slovo v zadaném řetězci. Pokud není nalezeno
žádné slovo, je vrácen prázdný řetězec. [1 bod]

f) Funkce vracející nejčastěji se vyskytující slovo v zadaném řetězci. Pokud se vyskytují dvě slova
stejně často, je vráceno slovo, které se v řetězci vyskytuje jako první. Pokud není nalezeno žádné
slovo, je vrácen prázdný řetězec. Dejte pozor na situace, kdy jedno slovo je podslovem jiného
slova. V řetězci "Ja a Jana" má slovo "a" pouze jeden výskyt. [4 body]


Ukázka:

string s1 = "Alena ma 2 ruce.";

int ruznych = RuznychPismen(s1); // 8

int pocet = PocetSlov(s1); // 3

string s2 = PrvniSlovo(s1, 2); // "Alena"

string s3 = PrvniSlovo(s1, s1.Length - 1); // ""

string s4 = NejdelsiSlovo(s1); // "Alena"

string s5 = NejvetsiSlovo(s1); // "ruce"

string s6 = "Mama ma 2 ruce a v ruce hul";

string s7 = NejcastejsiSlovo(s1 + s6); // "ruce"

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

// Проверяем, является ли байт английской буквой.
//
// b'A'..=b'Z' — заглавные буквы.
// b'a'..=b'z' — строчные буквы.
//
// Здесь используем байты, поскольку английские буквы
// занимают ровно один байт в UTF-8.
fn is_letter(character: u8) -> bool {
    (character >= b'A' && character <= b'Z') || (character >= b'a' && character <= b'z')
}

// Преобразуем заглавную английскую букву в строчную.
//
// Например:
// 'A' -> 'a'
// 'B' -> 'b'
//
// Остальные символы оставляем без изменений.
fn to_lower(character: u8) -> u8 {
    if character >= b'A' && character <= b'Z' {
        return character + 32;
    }

    character
}

// Сравниваем два слова без учёта регистра.
//
// Не создаём новые строки.
// Сравниваем символы непосредственно в исходных строках.
fn words_equal(first: &str, second: &str) -> bool {
    let a = first.as_bytes();
    let b = second.as_bytes();

    if a.len() != b.len() {
        return false;
    }

    let mut i = 0;

    while i < a.len() {
        if to_lower(a[i]) != to_lower(b[i]) {
            return false;
        }

        i += 1;
    }

    true
}

// Проверяем, является ли первое слово
// лексикографически больше второго.
//
// Например:
// "ruce" > "ma"  -> true
// "Alena" > "ruce" -> false
//
// Сравниваем без учёта регистра.
fn word_is_greater(first: &str, second: &str) -> bool {
    let a = first.as_bytes();
    let b = second.as_bytes();

    let mut i = 0;

    // Сравниваем буквы, пока они есть в обоих словах.
    while i < a.len() && i < b.len() {
        let letter_a = to_lower(a[i]);
        let letter_b = to_lower(b[i]);

        if letter_a > letter_b {
            return true;
        }

        if letter_a < letter_b {
            return false;
        }

        i += 1;
    }

    // Если все общие буквы одинаковые,
    // более длинное слово считается большим.
    //
    // Например: "apple" > "app".
    a.len() > b.len()
}

// A) Количество различных букв английского алфавита.
//
// Например:
// "Alena ma 2 ruce." -> 8
//
// Каждую букву проверяем только один раз,
// независимо от количества её повторений.
fn count_distinct_letters(text: &str) -> i32 {
    let bytes = text.as_bytes();

    // Для каждой буквы проверяем,
    // встретилась ли она раньше в строке.
    let mut count = 0;
    let mut i = 0;

    while i < bytes.len() {
        if is_letter(bytes[i]) {
            let current = to_lower(bytes[i]);

            let mut found_before = false;
            let mut j = 0;

            // Проверяем все предыдущие символы.
            while j < i {
                if is_letter(bytes[j]) && to_lower(bytes[j]) == current {
                    found_before = true;
                    break;
                }

                j += 1;
            }

            // Если такой буквы раньше не было,
            // увеличиваем количество различных букв.
            if found_before == false {
                count += 1;
            }
        }

        i += 1;
    }

    count
}

// B) Количество слов в строке.
//
// Слово начинается там, где находится буква,
// а предыдущий символ не является буквой.
//
// Например:
// "Alena ma 2 ruce." -> 3
fn count_words(text: &str) -> i32 {
    let bytes = text.as_bytes();

    let mut count = 0;
    let mut i = 0;

    while i < bytes.len() {
        if is_letter(bytes[i]) {
            // Нашли начало нового слова.
            count += 1;

            // Пропускаем остальные буквы слова.
            while i < bytes.len() && is_letter(bytes[i]) {
                i += 1;
            }
        } else {
            // Пропускаем разделитель.
            i += 1;
        }
    }

    count
}

// C) Первое слово начиная с заданной позиции.
//
// Если позиция находится внутри слова,
// возвращаем это слово целиком.
//
// Если позиция указывает на разделитель,
// ищем следующее слово справа.
//
// Если слов больше нет, возвращаем "".
fn first_word(text: &str, position: usize) -> &str {
    let bytes = text.as_bytes();

    // Позиция находится за пределами строки.
    if position >= bytes.len() {
        return "";
    }

    let mut start = position;

    if is_letter(bytes[start]) {
        // Позиция внутри слова.
        // Идём влево до начала слова.
        while start > 0 && is_letter(bytes[start - 1]) {
            start -= 1;
        }
    } else {
        // Позиция на разделителе.
        // Ищем ближайшую букву справа.
        while start < bytes.len() && is_letter(bytes[start]) == false {
            start += 1;
        }
    }

    // Если справа не оказалось слова.
    if start == bytes.len() {
        return "";
    }

    // Находим конец найденного слова.
    let mut end = start;

    while end < bytes.len() && is_letter(bytes[end]) {
        end += 1;
    }

    // Возвращаем часть исходной строки.
    // Новую строку не создаём.
    &text[start..end]
}

// D) Самое длинное слово.
//
// Если несколько слов имеют одинаковую длину,
// возвращаем первое из них.
fn longest_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    let mut longest = "";
    let mut i = 0;

    while i < bytes.len() {
        if is_letter(bytes[i]) {
            let start = i;

            // Находим конец текущего слова.
            while i < bytes.len() && is_letter(bytes[i]) {
                i += 1;
            }

            let word = &text[start..i];

            // Заменяем результат только если
            // нашли строго более длинное слово.
            if word.len() > longest.len() {
                longest = word;
            }
        } else {
            i += 1;
        }
    }

    longest
}

// E) Лексикографически наибольшее слово.
//
// Сравниваем слова по алфавиту без учёта регистра.
//
// Например:
// "Alena ma 2 ruce." -> "ruce"
fn greatest_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    let mut greatest = "";
    let mut i = 0;

    while i < bytes.len() {
        if is_letter(bytes[i]) {
            let start = i;

            while i < bytes.len() && is_letter(bytes[i]) {
                i += 1;
            }

            let word = &text[start..i];

            // Если это первое слово или оно больше
            // предыдущего результата — запоминаем.
            if greatest.is_empty() || word_is_greater(word, greatest) {
                greatest = word;
            }
        } else {
            i += 1;
        }
    }

    greatest
}

// F) Самое частое слово.
//
// Для каждого слова считаем, сколько раз
// оно встречается во всей строке.
//
// Сравниваем полные слова, а не подстроки.
//
// Например:
// "Ja a Jana"
//
// Слово "a" встречается только один раз,
// а не внутри слов "Ja" и "Jana".
//
// При одинаковой частоте возвращаем слово,
// которое встретилось раньше.
fn most_frequent_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    let mut most_frequent = "";
    let mut max_count = 0;
    let mut i = 0;

    // Перебираем все слова исходной строки.
    while i < bytes.len() {
        if is_letter(bytes[i]) {
            let start = i;

            while i < bytes.len() && is_letter(bytes[i]) {
                i += 1;
            }

            let current_word = &text[start..i];

            // Считаем, сколько раз текущее слово
            // встречается во всём исходном тексте.
            let mut count = 0;
            let mut j = 0;

            while j < bytes.len() {
                if is_letter(bytes[j]) {
                    let word_start = j;

                    // Находим конец очередного слова.
                    while j < bytes.len() && is_letter(bytes[j]) {
                        j += 1;
                    }

                    let other_word = &text[word_start..j];

                    // Сравниваем слова полностью,
                    // без учёта регистра.
                    if words_equal(current_word, other_word) {
                        count += 1;
                    }
                } else {
                    j += 1;
                }
            }

            // Обновляем результат только если частота
            // строго больше предыдущего максимума.
            //
            // Поэтому при равной частоте
            // сохраняется первое встретившееся слово.
            if count > max_count {
                max_count = count;
                most_frequent = current_word;
            }
        } else {
            i += 1;
        }
    }

    most_frequent
}

// MAIN
fn main() {
    // Исходные строки из задания.
    let text1 = "Alena ma 2 ruce.";

    println!("STRING OPERATIONS");
    println!("========================================");
    println!("Original text: {:?}", text1);
    println!();

    // A) Различные буквы английского алфавита.
    println!("A) Distinct letters: {}", count_distinct_letters(text1));

    // B) Количество слов.
    println!("B) Word count: {}", count_words(text1));

    // C) Поиск слова по позиции.
    println!("C) Word at position 2: {:?}", first_word(text1, 2));

    println!(
        "   Word at last position: {:?}",
        first_word(text1, text1.len() - 1)
    );

    // D) Самое длинное слово.
    println!("D) Longest word: {:?}", longest_word(text1));

    // E) Лексикографически наибольшее слово.
    println!("E) Greatest word: {:?}", greatest_word(text1));

    // F) Самое частое слово.
    //
    // В примере преподавателя строки соединяются без пробела:
    // "Alena ma 2 ruce.Mama ma 2 ruce a v ruce hul"
    //
    // Используем concat! — объединение строковых литералов
    // во время компиляции, без создания копии во время выполнения.
    let combined = "Alena ma 2 ruce.Mama ma 2 ruce a v ruce hul";
    println!();
    println!("Combined text: {:?}", combined);

    println!("F) Most frequent word: {:?}", most_frequent_word(combined));

    // Дополнительная проверка:
    // подстроки нельзя считать отдельными словами.
    let test = "Ja a Jana";

    println!();
    println!("SUBSTRING TEST");
    println!("Text: {:?}", test);
    println!("Most frequent word: {:?}", most_frequent_word(test));

    // Проверяем пустую строку.
    println!();
    println!("EMPTY STRING TEST");
    println!("Word count: {}", count_words(""));
    println!("Longest word: {:?}", longest_word(""));
    println!("Greatest word: {:?}", greatest_word(""));
    println!("Most frequent word: {:?}", most_frequent_word(""));
}
