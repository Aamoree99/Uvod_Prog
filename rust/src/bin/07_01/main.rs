/*
Cvičení 07 — Úloha 7.1
Název: Dlouhá celá čísla
Body: 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 2 body; 3 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s dlouhými celými čísly, které používají způsob uložení cifer v poli big
endian (na přednášce byl použit způsob little endian).

a) Funkci Vytvor, která převede textový řetězec na číslo. [1 bod]

b) Funkci Vypis, která vypíše číslo na obrazovku bez zbytečných nul. [1 bod]

c) Funkce Compare porovná dvě čísla. Je-li první číslo menší než druhé číslo, je vrácena hodnota
-1. Je-li první číslo větší než druhé číslo, je vrácena hodnota 1. Jsou-li si obě čísla rovna, je
vrácena hodnota 0. [1 bod]

d) Funkce Soucet sečte dvě čísla. [1 bod]

e) Funkce Rozdil odečte menší číslo od většího. [1 bod]

f) Funkce Soucin vynásobí dvě čísla. [2 body]

g) Funkce Podil vydělí číslo číslem. [3 body]


Ukázka:

int[] c1 = Vytvor("560111202232646");

int[] c2 = Vytvor("560100600690986");

Vypis(c1); // 560111202232646

int i = Compare(c1, c2); // 1

int[] c3 = Soucet(c1, c2);  // 1120211802923632

int[] c4 = Rozdil(c1, c2);  // 10601541660

int[] c5 = Soucin(c1, c4); // 5938042244702081581032360

int[] c6 = Podil(c1, c4); // 52832

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

// Максимальное количество цифр в числе.
const MAX_DIGITS: usize = 100;

// Каждое число хранится в 100 ячейках.
// Неиспользуемые ячейки слева заполнены нулями.
type BigNumber = [u8; MAX_DIGITS];

// --------------------------------------------------
// A) CREATE NUMBER
// --------------------------------------------------

// Преобразуем текст в массив цифр.
//
// "56012" превращается в:
// [0, 0, ... 0, 5, 6, 0, 1, 2]
fn create_number(text: &str) -> BigNumber {
    let mut result = [0; MAX_DIGITS];
    let bytes = text.as_bytes();

    // Проверяем длину числа.
    assert!(bytes.len() > 0, "Number cannot be empty");
    assert!(bytes.len() <= MAX_DIGITS, "Number is too long");

    // Сдвиг, с которого начинаем записывать цифры.
    let offset = MAX_DIGITS - bytes.len();

    let mut i = 0;

    while i < bytes.len() {
        // Разрешаем только цифры от 0 до 9.
        assert!(bytes[i] >= b'0' && bytes[i] <= b'9', "Invalid digit");

        // Преобразуем ASCII-символ в число.
        result[offset + i] = bytes[i] - b'0';

        i += 1;
    }

    result
}

// --------------------------------------------------
// B) PRINT NUMBER
// --------------------------------------------------

// Печатаем число без ведущих нулей.
//
// Например:
// [0, 0, 0, 5, 6] -> 56
//
// Если все цифры равны нулю, выводим один 0.
fn print_number(number: &[u8]) {
    let mut i = 0;

    // Пропускаем нули в начале.
    while i + 1 < number.len() && number[i] == 0 {
        i += 1;
    }

    // Печатаем оставшиеся цифры.
    while i < number.len() {
        print!("{}", number[i]);
        i += 1;
    }

    println!();
}

// --------------------------------------------------
// C) COMPARE
// --------------------------------------------------

// Сравниваем два числа одинаковой длины.
//
// Возвращаем:
// -1 — первое меньше второго.
//  0 — числа равны.
//  1 — первое больше второго.
//
// Используется также для вспомогательных
// массивов при делении.
fn compare(first: &[u8], second: &[u8]) -> i32 {
    assert!(first.len() == second.len(), "Different array lengths");

    let mut i = 0;

    // В Big Endian самые важные цифры находятся слева.
    while i < first.len() {
        if first[i] < second[i] {
            return -1;
        }

        if first[i] > second[i] {
            return 1;
        }

        i += 1;
    }

    0
}

// --------------------------------------------------
// D) ADDITION
// --------------------------------------------------

// Складываем два больших числа столбиком.
//
// Идём справа налево и переносим десятки
// в следующий разряд.
fn add(first: &BigNumber, second: &BigNumber) -> BigNumber {
    let mut result = [0; MAX_DIGITS];

    // Перенос в следующий разряд.
    let mut carry: u32 = 0;

    let mut i = MAX_DIGITS;

    while i > 0 {
        i -= 1;

        // Складываем две цифры и перенос.
        let sum = first[i] as u32 + second[i] as u32 + carry;

        // Последняя цифра суммы.
        result[i] = (sum % 10) as u8;

        // Перенос десятков.
        carry = sum / 10;
    }

    // Если после последней цифры остался перенос,
    // результат не помещается в 100 цифр.
    assert!(carry == 0, "Addition overflow");

    result
}

// --------------------------------------------------
// E) SUBTRACTION
// --------------------------------------------------

// Вспомогательная функция для вычитания.
//
// Вычитает second из first прямо в массиве first.
// Предполагается, что first >= second.
//
// Работает с массивами одинаковой длины.
fn subtract_in_place(first: &mut [u8], second: &[u8]) {
    assert!(compare(first, second) >= 0, "Negative result");

    // Заимствование десятки.
    let mut borrow = 0;

    let mut i = first.len();

    while i > 0 {
        i -= 1;

        let mut difference = first[i] as i32 - second[i] as i32 - borrow;

        if difference < 0 {
            // Занимаем десятку из следующего разряда.
            difference += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        first[i] = difference as u8;
    }
}

// Вычитаем меньшее число из большего.
//
// Результат всегда неотрицательный.
fn subtract(first: &BigNumber, second: &BigNumber) -> BigNumber {
    let mut result;

    if compare(first, second) >= 0 {
        result = *first;
        subtract_in_place(&mut result, second);
    } else {
        result = *second;
        subtract_in_place(&mut result, first);
    }

    result
}

// --------------------------------------------------
// F) MULTIPLICATION
// --------------------------------------------------

// Умножаем два числа столбиком.
//
// Для умножения двух чисел по 100 цифр
// может потребоваться 200 цифр.
//
// Поэтому сначала считаем результат в массиве
// из 200 элементов.
//
// Если ответ не помещается в 100 цифр,
// программа сообщит об ошибке.
fn multiply(first: &BigNumber, second: &BigNumber) -> BigNumber {
    let mut temporary = [0u8; MAX_DIGITS * 2];

    let mut i = MAX_DIGITS;

    // Берём каждую цифру первого числа справа налево.
    while i > 0 {
        i -= 1;

        let mut carry: u32 = 0;
        let mut j = MAX_DIGITS;

        // Умножаем её на каждую цифру второго числа.
        while j > 0 {
            j -= 1;

            // Позиция соответствующего разряда
            // в массиве из 200 цифр.
            let position = i + j + 1;

            let product = first[i] as u32 * second[j] as u32 + temporary[position] as u32 + carry;

            temporary[position] = (product % 10) as u8;
            carry = product / 10;
        }

        // Записываем оставшийся перенос.
        temporary[i] = carry as u8;
    }

    // Проверяем, помещается ли произведение
    // в наши 100 цифр.
    let mut i = 0;

    while i < MAX_DIGITS {
        assert!(temporary[i] == 0, "Multiplication overflow");
        i += 1;
    }

    // Копируем последние 100 цифр.
    let mut result = [0; MAX_DIGITS];
    i = 0;

    while i < MAX_DIGITS {
        result[i] = temporary[MAX_DIGITS + i];
        i += 1;
    }

    result
}

// --------------------------------------------------
// G) DIVISION
// --------------------------------------------------

// Целочисленное деление столбиком.
//
// first / second
//
// Возвращаем только целую часть частного.
//
// Например:
// 1234 / 12 = 102
//
// Массив остатка имеет 101 цифру,
// чтобы избежать переполнения при сносе
// следующей цифры во время деления.
fn divide(first: &BigNumber, second: &BigNumber) -> BigNumber {
    let zero = [0; MAX_DIGITS];

    // Деление на ноль запрещено.
    assert!(compare(second, &zero) != 0, "Division by zero");

    let mut result = [0; MAX_DIGITS];

    // Текущий остаток при делении.
    let mut remainder = [0u8; MAX_DIGITS + 1];

    // Делитель тоже расширяем до 101 цифры,
    // добавляя один ноль слева.
    let mut divisor = [0u8; MAX_DIGITS + 1];

    let mut i = 0;

    while i < MAX_DIGITS {
        divisor[i + 1] = second[i];
        i += 1;
    }

    // Последовательно сносим цифры делимого.
    i = 0;

    while i < MAX_DIGITS {
        // Сдвигаем остаток влево на один разряд.
        // Это эквивалентно умножению на 10.
        let mut j = 0;

        while j < MAX_DIGITS {
            remainder[j] = remainder[j + 1];
            j += 1;
        }

        // Сносим очередную цифру делимого.
        remainder[MAX_DIGITS] = first[i];

        // Цифра частного находится в диапазоне 0..9.
        let mut quotient_digit = 0;

        // Вычитаем делитель, пока остаток
        // не станет меньше делителя.
        while compare(&remainder, &divisor) >= 0 {
            subtract_in_place(&mut remainder, &divisor);
            quotient_digit += 1;
        }

        // Записываем цифру частного.
        result[i] = quotient_digit;

        i += 1;
    }

    result
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    // Числа из задания преподавателя.
    let first = create_number("560111202232646");
    let second = create_number("560100600690986");

    println!("BIG INTEGER OPERATIONS");
    println!("========================================");

    // A) Создание чисел из строк.
    println!("A) Numbers created successfully.");

    // B) Вывод без лишних нулей.
    print!("B) First number:  ");
    print_number(&first);

    print!("   Second number: ");
    print_number(&second);

    // C) Сравнение.
    let comparison = compare(&first, &second);

    println!();
    println!("C) Comparison: {}", comparison);

    // D) Сложение.
    let sum = add(&first, &second);

    print!("D) Sum: ");
    print_number(&sum);

    // E) Вычитание.
    let difference = subtract(&first, &second);

    print!("E) Difference: ");
    print_number(&difference);

    // F) Умножение.
    // По заданию умножаем первое число на разность.
    let product = multiply(&first, &difference);

    print!("F) Product: ");
    print_number(&product);

    // G) Деление.
    // По заданию делим первое число на разность.
    let quotient = divide(&first, &difference);

    print!("G) Quotient: ");
    print_number(&quotient);

    // Дополнительные проверки нуля и ведущих нулей.
    println!();
    println!("ADDITIONAL TESTS");

    let zero = create_number("0000");
    let smaller = create_number("25");
    let larger = create_number("100");

    print!("Zero: ");
    print_number(&zero);

    print!("25 + 100 = ");
    print_number(&add(&smaller, &larger));

    print!("100 - 25 = ");
    print_number(&subtract(&larger, &smaller));

    print!("25 * 100 = ");
    print_number(&multiply(&smaller, &larger));

    print!("100 / 25 = ");
    print_number(&divide(&larger, &smaller));
}
