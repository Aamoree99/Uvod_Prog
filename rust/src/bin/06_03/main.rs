/*
Cvičení 06 — Úloha 6.3
Název: Naše vlastní řetězce
Body: 2 body; 2 body; 1 bod; 2 body; 2 body; 4 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s polem znaků char[], které budou mít stejnou funkčnost jako funkce pro
textové řetězce string. (Aniž byste pole znaků char[], na textový řetězec string převedli).

a) Funkce lexikografického porovnání řetězců s1 a s2: Compare(s1, s2). [2 body]

b) Funkce vracející index prvního výskytu hledaného řetězce s2 v řetězci s1. Řetězec s1 se
prohledává od zadané pozice start: IndexOf(s1, s2, start). [2 body]

c) Funkce vracející podřetězec zadaného řetězce s1. Podřetězec začíná na zadané pozici a má zadanou
délku: Substring(s1, start, delka). [1 bod]

d) Funkce vracející řetězec vzniklý vložením řetězce s2 do řetězce s1 na pozici zadanou indexem:
Insert(s1, index, s2). [2 body]

e) Funkce vracející řetězec vzniklý odstraněním podřetězce ze zadaného řetězce s1. Z řetězce s1 se
odstraní podřetězec určený indexem (v rámci řetězce s1) svého prvního znaku a svojí délkou:
Remove(s1, index, delka). [2 body]

f) Funkce vracející řetězec vzniklý z řetězce s1 nahrazením všech výskytů řetězce s2 za řetězec s3:
Replace (s1, s2, s3). [4 body]


Ukázka:

char[] s1 = "klokan".ToCharArray();

char[] s2 = "klobouk".ToCharArray();

int i1 = Compare(s1, s2); // 1

int i2 = IndexOf(s2, "o".ToCharArray(), 3); // 4

char[] s3 = Substring(s2, 2, 3); // 'o''b''o'

char[] s4 = Insert(s1, 1, s3); // 'k''o''b''o''l''o''k''a''n'

char[] s5 = Remove(s2, 3, 2); // 'k''l''o''u''k''

char[] s6 = Replace(s5, "k".ToCharArray(), s3);

// 'o''b''o''l''o''u''o''b''o'

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

// Максимальная вместимость массива символов.
const MAX_LENGTH: usize = 100;

// A) COMPARE
//
// Лексикографически сравниваем два массива символов.
//
// Возвращаем:
// -1, если first < second
//  0, если first == second
//  1, если first > second
//
// Регистр учитывается, как при обычном сравнении строк.
fn compare(first: &[char], second: &[char]) -> i32 {
    let mut i = 0;

    // Сравниваем символы на одинаковых позициях.
    while i < first.len() && i < second.len() {
        if first[i] < second[i] {
            return -1;
        }

        if first[i] > second[i] {
            return 1;
        }

        i += 1;
    }

    // Если общие символы одинаковы,
    // более длинная строка считается большей.
    if first.len() < second.len() {
        return -1;
    }

    if first.len() > second.len() {
        return 1;
    }

    0
}

// B) INDEX OF
//
// Ищем первое полное совпадение массива search
// внутри массива array, начиная с позиции start.
//
// Возвращаем индекс найденного совпадения.
// Если совпадений нет, возвращаем -1.
fn index_of(array: &[char], search: &[char], start: usize) -> isize {
    // Не начинаем поиск за пределами массива.
    if start > array.len() {
        return -1;
    }

    // Пустая искомая строка считается найденной
    // на начальной позиции.
    if search.len() == 0 {
        return start as isize;
    }

    // Если искомая строка длиннее оставшейся части.
    if search.len() > array.len() - start {
        return -1;
    }

    let mut i = start;

    // Проверяем каждую возможную начальную позицию.
    while i <= array.len() - search.len() {
        let mut j = 0;

        // Сравниваем символы по очереди.
        while j < search.len() && array[i + j] == search[j] {
            j += 1;
        }

        // Если совпали все символы, слово найдено.
        if j == search.len() {
            return i as isize;
        }

        i += 1;
    }

    -1
}

// C) SUBSTRING
//
// Копируем часть массива:
// start — начальная позиция;
// length — количество символов.
//
// Возвращаем массив и его фактическую длину.
//
// Например:
// ['k','l','o','b','o','u','k']
// start = 2, length = 3
// результат: ['o','b','o']
fn substring(array: &[char], start: usize, length: usize) -> ([char; MAX_LENGTH], usize) {
    let mut result = ['\0'; MAX_LENGTH];

    // Проверяем границы и вместимость результата.
    assert!(start <= array.len(), "Invalid start index");
    assert!(length <= array.len() - start, "Invalid substring length");
    assert!(length <= MAX_LENGTH, "Result is too long");

    let mut i = 0;

    while i < length {
        result[i] = array[start + i];
        i += 1;
    }

    (result, length)
}

// D) INSERT
//
// Вставляем массив insert_array в исходный массив
// на указанную позицию index.
//
// Сначала копируем часть исходного массива до index,
// затем вставляем новые символы,
// после чего копируем оставшуюся часть.
fn insert(array: &[char], index: usize, insert_array: &[char]) -> ([char; MAX_LENGTH], usize) {
    let mut result = ['\0'; MAX_LENGTH];

    assert!(index <= array.len(), "Invalid insertion index");
    assert!(
        array.len() + insert_array.len() <= MAX_LENGTH,
        "Result is too long"
    );

    let mut position = 0;
    let mut i = 0;

    // Копируем начало исходного массива.
    while i < index {
        result[position] = array[i];
        position += 1;
        i += 1;
    }

    // Добавляем вставляемые символы.
    i = 0;

    while i < insert_array.len() {
        result[position] = insert_array[i];
        position += 1;
        i += 1;
    }

    // Копируем остаток исходного массива.
    i = index;

    while i < array.len() {
        result[position] = array[i];
        position += 1;
        i += 1;
    }

    (result, position)
}

// E) REMOVE
//
// Удаляем из массива length символов,
// начиная с позиции index.
//
// Копируем все символы, кроме удаляемого участка.
fn remove(array: &[char], index: usize, length: usize) -> ([char; MAX_LENGTH], usize) {
    let mut result = ['\0'; MAX_LENGTH];

    assert!(index <= array.len(), "Invalid removal index");
    assert!(length <= array.len() - index, "Invalid removal length");
    assert!(array.len() - length <= MAX_LENGTH, "Result is too long");

    let mut position = 0;
    let mut i = 0;

    while i < array.len() {
        // Копируем символы до удаляемого участка
        // и после него.
        if i < index || i >= index + length {
            result[position] = array[i];
            position += 1;
        }

        i += 1;
    }

    (result, position)
}

// F) REPLACE
//
// Заменяем все непересекающиеся вхождения search
// на символы replacement.
//
// При совпадении записываем replacement
// и пропускаем search.len() исходных символов.
//
// Например:
// "klouk", search = "k", replacement = "obo"
// результат: "obolouobo"
fn replace(array: &[char], search: &[char], replacement: &[char]) -> ([char; MAX_LENGTH], usize) {
    let mut result = ['\0'; MAX_LENGTH];

    // Пустую строку поиска не заменяем.
    assert!(search.len() > 0, "Search array cannot be empty");

    let mut i = 0;
    let mut position = 0;

    while i < array.len() {
        let mut matches = false;

        // Проверяем, помещается ли искомая строка
        // в оставшуюся часть исходного массива.
        if search.len() <= array.len() - i {
            matches = true;

            let mut j = 0;

            // Проверяем полное совпадение.
            while j < search.len() {
                if array[i + j] != search[j] {
                    matches = false;
                    break;
                }

                j += 1;
            }
        }

        if matches == true {
            // Если нашли совпадение,
            // записываем вместо него replacement.
            let mut j = 0;

            while j < replacement.len() {
                assert!(position < MAX_LENGTH, "Result is too long");

                result[position] = replacement[j];
                position += 1;
                j += 1;
            }

            // Пропускаем заменённую часть массива.
            i += search.len();
        } else {
            // Совпадения нет — копируем один символ.
            assert!(position < MAX_LENGTH, "Result is too long");

            result[position] = array[i];
            position += 1;
            i += 1;
        }
    }

    (result, position)
}

// Выводим массив символов без квадратных скобок,
// запятых и служебных символов '\0'.
//
// Никаких String не создаём.
fn print_chars(array: &[char]) {
    let mut i = 0;

    while i < array.len() {
        print!("{}", array[i]);
        i += 1;
    }

    println!();
}

// MAIN
fn main() {
    // Исходные массивы из задания.
    let s1 = ['k', 'l', 'o', 'k', 'a', 'n'];
    let s2 = ['k', 'l', 'o', 'b', 'o', 'u', 'k'];

    println!("CUSTOM STRING OPERATIONS");
    println!("========================================");

    print!("First array:  ");
    print_chars(&s1);

    print!("Second array: ");
    print_chars(&s2);

    // A) Сравниваем два массива символов.
    let comparison = compare(&s1, &s2);

    println!();
    println!("A) Compare: {}", comparison);

    // B) Ищем символ 'o' начиная с индекса 3.
    let search = ['o'];
    let found_index = index_of(&s2, &search, 3);

    println!("B) IndexOf 'o' from position 3: {}", found_index);

    // C) Получаем три символа начиная с индекса 2.
    let (s3, s3_length) = substring(&s2, 2, 3);

    print!("C) Substring: ");
    print_chars(&s3[..s3_length]);

    // D) Вставляем найденный подмассив в s1.
    let (s4, s4_length) = insert(&s1, 1, &s3[..s3_length]);

    print!("D) Insert: ");
    print_chars(&s4[..s4_length]);

    // E) Удаляем два символа из s2 начиная с индекса 3.
    let (s5, s5_length) = remove(&s2, 3, 2);

    print!("E) Remove: ");
    print_chars(&s5[..s5_length]);

    // F) Заменяем все символы 'k' в результате удаления
    // на массив ['o', 'b', 'o'].
    let search_k = ['k'];

    let (s6, s6_length) = replace(&s5[..s5_length], &search_k, &s3[..s3_length]);

    print!("F) Replace: ");
    print_chars(&s6[..s6_length]);
}
