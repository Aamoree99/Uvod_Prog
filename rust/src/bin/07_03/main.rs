/*
Cvičení 07 — Úloha 7.3
Název: Zlomky
Body: 1 bod; 1 bod; 2 body; 2 body; 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Napište funkce pracující se zlomky, které mají čitatele i jmenovatele reprezentované dlouhým celým
číslem. Využijte funkce z příkladu 7.1. Výsledky funkcí d až f budou v základním tvaru.

a) Funkce VytvorZlomek načte zlomek z řetězce. [1 bod]

b) Funkce VypisZlomek vypíše zlomek na obrazovku. [1 bod]

c) Funkce ZakladniTvar převede zlomek na základní tvar. [2 body]

d) Funkce SoucetZlomku sečte dva zlomky. [2 body]

e) Funkce SoucinZlomku vynásobí dva zlomky [1 bod]

f) Funkce PodilZlomku vydělí zlomek zlomkem. [1 bod]


Ukázka:

Zlomek z1 = VytvorZlomek("-56890/1223366");

Zlomek z2 = VytvorZlomek("2565689841/2365");

VypisZlomek(z1); VypisZlomek(z2);

ZakladniTvar(ref z1); // -28445/611683

ZakladniTvar(ref z2); // 233244531/215

VypisZlomek(z1); VypisZlomek(z2);

Zlomek z3 = SoucetZlomku(z1, z2); // 142671708339998/131511845

Zlomek z4 = SoucinZlomku(z1, z2); // -1326928136859/26302369

Zlomek z5 = PodilZlomku(z2, z1); // -142671714455673/6115675

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

// Максимальное количество десятичных цифр.
const MAX_DIGITS: usize = 100;

// Длинное целое число.
// Цифры хранятся в Big Endian:
//
// 123 -> [0, 0, ... 0, 1, 2, 3]
type BigNumber = [u8; MAX_DIGITS];

// Дробь состоит из двух длинных чисел и знака.
//
// numerator — абсолютное значение числителя.
// denominator — положительный знаменатель.
// negative — знак всей дроби.
#[derive(Clone, Copy)]
struct Fraction {
    numerator: BigNumber,
    denominator: BigNumber,
    negative: bool,
}

// --------------------------------------------------
// BIG INTEGER: BASIC FUNCTIONS
// --------------------------------------------------

// Создаём нулевое длинное число.
fn zero() -> BigNumber {
    [0; MAX_DIGITS]
}

// Создаём длинное число со значением 1.
fn one() -> BigNumber {
    let mut result = zero();
    result[MAX_DIGITS - 1] = 1;
    result
}

// Проверяем, является ли длинное число нулём.
fn is_zero(number: &BigNumber) -> bool {
    let mut i = 0;

    while i < MAX_DIGITS {
        if number[i] != 0 {
            return false;
        }

        i += 1;
    }

    true
}

// Преобразуем строку с цифрами в длинное число.
//
// Знак здесь не обрабатываем.
// Для дробей знак хранится отдельно.
fn create_big_number(text: &str) -> BigNumber {
    let bytes = text.as_bytes();

    assert!(bytes.len() > 0, "Empty number");
    assert!(bytes.len() <= MAX_DIGITS, "Number is too long");

    let mut result = zero();

    // Выравниваем цифры по правому краю массива.
    let offset = MAX_DIGITS - bytes.len();

    let mut i = 0;

    while i < bytes.len() {
        assert!(bytes[i] >= b'0' && bytes[i] <= b'9', "Invalid digit");

        result[offset + i] = bytes[i] - b'0';

        i += 1;
    }

    result
}

// Выводим длинное число без ведущих нулей.
fn print_big_number(number: &BigNumber) {
    let mut i = 0;

    // Пропускаем нули слева, оставляя хотя бы одну цифру.
    while i + 1 < MAX_DIGITS && number[i] == 0 {
        i += 1;
    }

    while i < MAX_DIGITS {
        print!("{}", number[i]);
        i += 1;
    }
}

// --------------------------------------------------
// BIG INTEGER: COMPARISON
// --------------------------------------------------

// Сравниваем два длинных числа.
//
// -1 — первое меньше второго.
//  0 — числа равны.
//  1 — первое больше второго.
fn compare_big(first: &BigNumber, second: &BigNumber) -> i32 {
    let mut i = 0;

    while i < MAX_DIGITS {
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
// BIG INTEGER: ADDITION
// --------------------------------------------------

// Складываем два длинных числа столбиком.
//
// Перебираем цифры справа налево.
// carry хранит перенос в следующий разряд.
fn add_big(first: &BigNumber, second: &BigNumber) -> BigNumber {
    let mut result = zero();

    let mut carry = 0u16;
    let mut i = MAX_DIGITS;

    while i > 0 {
        i -= 1;

        let sum = first[i] as u16 + second[i] as u16 + carry;

        result[i] = (sum % 10) as u8;
        carry = sum / 10;
    }

    assert!(carry == 0, "Addition overflow");

    result
}

// --------------------------------------------------
// BIG INTEGER: SUBTRACTION
// --------------------------------------------------

// Вычитаем второе число из первого.
//
// Требование: first >= second.
//
// borrow показывает, заняли ли десятку
// из следующего разряда.
fn subtract_big(first: &BigNumber, second: &BigNumber) -> BigNumber {
    assert!(compare_big(first, second) >= 0, "Negative subtraction");

    let mut result = zero();

    let mut borrow = 0i32;
    let mut i = MAX_DIGITS;

    while i > 0 {
        i -= 1;

        let mut difference = first[i] as i32 - second[i] as i32 - borrow;

        if difference < 0 {
            difference += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        result[i] = difference as u8;
    }

    result
}

// --------------------------------------------------
// BIG INTEGER: MULTIPLICATION
// --------------------------------------------------

// Умножаем два длинных числа столбиком.
//
// Промежуточный массив имеет 200 цифр,
// потому что произведение двух 100-значных чисел
// может содержать до 200 цифр.
//
// Итог должен помещаться в MAX_DIGITS.
fn multiply_big(first: &BigNumber, second: &BigNumber) -> BigNumber {
    let mut work = [0u32; MAX_DIGITS * 2];

    let mut i = MAX_DIGITS;

    // Перебираем цифры первого числа справа налево.
    while i > 0 {
        i -= 1;

        let mut j = MAX_DIGITS;

        // Умножаем на каждую цифру второго числа.
        while j > 0 {
            j -= 1;

            let position = i + j + 1;

            work[position] += first[i] as u32 * second[j] as u32;
        }
    }

    // Выполняем перенос десятков.
    let mut i = MAX_DIGITS * 2;

    while i > 1 {
        i -= 1;

        let carry = work[i] / 10;

        work[i] %= 10;
        work[i - 1] += carry;
    }

    assert!(work[0] < 10, "Multiplication overflow");

    // Проверяем, что старшие 100 цифр равны нулю.
    i = 0;

    while i < MAX_DIGITS {
        assert!(work[i] == 0, "Multiplication overflow");
        i += 1;
    }

    // Копируем младшие 100 цифр результата.
    let mut result = zero();
    i = 0;

    while i < MAX_DIGITS {
        result[i] = work[MAX_DIGITS + i] as u8;
        i += 1;
    }

    result
}

// --------------------------------------------------
// BIG INTEGER: DIVISION
// --------------------------------------------------

// Сравниваем два вспомогательных массива
// длиной MAX_DIGITS + 1.
//
// Дополнительная цифра нужна для деления столбиком.
fn compare_extended(first: &[u8; MAX_DIGITS + 1], second: &[u8; MAX_DIGITS + 1]) -> i32 {
    let mut i = 0;

    while i < MAX_DIGITS + 1 {
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

// Вычитаем один расширенный массив из другого.
//
// Результат записывается в first.
// Предполагается, что first >= second.
fn subtract_extended(first: &mut [u8; MAX_DIGITS + 1], second: &[u8; MAX_DIGITS + 1]) {
    let mut borrow = 0i32;
    let mut i = MAX_DIGITS + 1;

    while i > 0 {
        i -= 1;

        let mut difference = first[i] as i32 - second[i] as i32 - borrow;

        if difference < 0 {
            difference += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        first[i] = difference as u8;
    }
}

// Деление двух длинных чисел столбиком.
//
// Возвращаем два значения:
// quotient — целая часть частного.
// remainder — остаток.
//
// Например:
// 17 / 5 -> (3, 2)
fn divide_big(first: &BigNumber, second: &BigNumber) -> (BigNumber, BigNumber) {
    assert!(is_zero(second) == false, "Division by zero");

    let mut quotient = zero();

    // Расширенный текущий остаток.
    let mut remainder = [0u8; MAX_DIGITS + 1];

    // Расширенный делитель с дополнительным нулём слева.
    let mut divisor = [0u8; MAX_DIGITS + 1];

    let mut i = 0;

    while i < MAX_DIGITS {
        divisor[i + 1] = second[i];
        i += 1;
    }

    // Перебираем цифры делимого слева направо.
    i = 0;

    while i < MAX_DIGITS {
        // Сдвигаем остаток влево на один разряд.
        // Это эквивалентно умножению на 10.
        let mut j = 0;

        while j < MAX_DIGITS {
            remainder[j] = remainder[j + 1];
            j += 1;
        }

        // Сносим следующую цифру делимого.
        remainder[MAX_DIGITS] = first[i];

        // Находим очередную цифру частного.
        let mut digit = 0u8;

        while compare_extended(&remainder, &divisor) >= 0 {
            subtract_extended(&mut remainder, &divisor);
            digit += 1;
        }

        quotient[i] = digit;
        i += 1;
    }

    // Переносим остаток обратно в 100-значный массив.
    let mut final_remainder = zero();

    i = 0;

    while i < MAX_DIGITS {
        final_remainder[i] = remainder[i + 1];
        i += 1;
    }

    (quotient, final_remainder)
}

// Деление без остатка.
//
// Используем после сокращения дроби через НОД.
fn divide_exact(first: &BigNumber, second: &BigNumber) -> BigNumber {
    let (quotient, remainder) = divide_big(first, second);

    assert!(is_zero(&remainder), "Division is not exact");

    quotient
}

// --------------------------------------------------
// BIG INTEGER: GCD
// --------------------------------------------------

// Находим наибольший общий делитель (НОД).
//
// Используем алгоритм Евклида:
//
// gcd(a, b) = gcd(b, a % b)
//
// Например:
// gcd(18, 12)
// gcd(12, 6)
// gcd(6, 0) = 6
fn gcd_big(first: &BigNumber, second: &BigNumber) -> BigNumber {
    let mut a = *first;
    let mut b = *second;

    while is_zero(&b) == false {
        // Делим a на b и берём остаток.
        let (_, remainder) = divide_big(&a, &b);

        // Переходим к следующей паре чисел.
        a = b;
        b = remainder;
    }

    a
}

// --------------------------------------------------
// A) CREATE FRACTION
// --------------------------------------------------

// Преобразуем строку в дробь.
//
// Примеры:
// "12/18"
// "-56890/1223366"
// "2565689841/2365"
//
// Знак хранится отдельно.
// Знаменатель не может быть равен нулю.
fn create_fraction(text: &str) -> Fraction {
    let bytes = text.as_bytes();

    assert!(bytes.len() > 0, "Empty fraction");

    // Находим разделитель '/'.
    let mut slash = 0;

    while slash < bytes.len() && bytes[slash] != b'/' {
        slash += 1;
    }

    assert!(slash < bytes.len(), "Missing '/'");
    assert!(slash > 0, "Missing numerator");
    assert!(slash + 1 < bytes.len(), "Missing denominator");

    // Отдельно читаем знак числителя.
    let mut negative = false;
    let mut start = 0;

    if bytes[0] == b'-' {
        negative = true;
        start = 1;
    } else if bytes[0] == b'+' {
        start = 1;
    }

    assert!(start < slash, "Missing numerator");

    // Получаем части исходного текста без копирования.
    let numerator_text = &text[start..slash];
    let denominator_text = &text[slash + 1..];

    let numerator = create_big_number(numerator_text);
    let denominator = create_big_number(denominator_text);

    assert!(is_zero(&denominator) == false, "Denominator cannot be zero");

    // Нулевой числитель всегда имеет положительный знак.
    if is_zero(&numerator) {
        negative = false;
    }

    Fraction {
        numerator,
        denominator,
        negative,
    }
}

// --------------------------------------------------
// B) PRINT FRACTION
// --------------------------------------------------

// Выводим дробь в формате numerator/denominator.
//
// Например:
// -28445/611683
fn print_fraction(fraction: &Fraction) {
    if fraction.negative {
        print!("-");
    }

    print_big_number(&fraction.numerator);

    print!("/");

    print_big_number(&fraction.denominator);

    println!();
}

// --------------------------------------------------
// C) REDUCE FRACTION
// --------------------------------------------------

// Приводим дробь к несократимому виду.
//
// Находим НОД числителя и знаменателя,
// затем делим оба числа на найденный НОД.
//
// Например:
// 12/18 -> 2/3.
fn reduce_fraction(fraction: &mut Fraction) {
    // Ноль представляем как 0/1.
    if is_zero(&fraction.numerator) {
        fraction.denominator = one();
        fraction.negative = false;
        return;
    }

    // Находим НОД.
    let gcd = gcd_big(&fraction.numerator, &fraction.denominator);

    // Делим числитель и знаменатель на НОД.
    fraction.numerator = divide_exact(&fraction.numerator, &gcd);

    fraction.denominator = divide_exact(&fraction.denominator, &gcd);
}

// --------------------------------------------------
// D) ADD FRACTIONS
// --------------------------------------------------

// Складываем дроби по формуле:
//
// a/b + c/d = (a*d + c*b) / (b*d)
//
// Если знаки разные, выполняем вычитание
// абсолютных величин числителей.
//
// Результат автоматически сокращаем.
fn add_fractions(first: &Fraction, second: &Fraction) -> Fraction {
    // Первый числитель умножаем на второй знаменатель.
    let a = multiply_big(&first.numerator, &second.denominator);

    // Второй числитель умножаем на первый знаменатель.
    let b = multiply_big(&second.numerator, &first.denominator);

    // Общий знаменатель.
    let denominator = multiply_big(&first.denominator, &second.denominator);

    let numerator;
    let negative;

    if first.negative == second.negative {
        // При одинаковых знаках складываем модули.
        numerator = add_big(&a, &b);
        negative = first.negative;
    } else {
        // При разных знаках вычитаем меньший модуль.
        let comparison = compare_big(&a, &b);

        if comparison >= 0 {
            numerator = subtract_big(&a, &b);
            negative = first.negative;
        } else {
            numerator = subtract_big(&b, &a);
            negative = second.negative;
        }
    }

    let mut result = Fraction {
        numerator,
        denominator,
        negative,
    };

    // Результат должен быть в основном виде.
    reduce_fraction(&mut result);

    result
}

// --------------------------------------------------
// E) MULTIPLY FRACTIONS
// --------------------------------------------------

// Умножаем дроби:
//
// a/b * c/d = (a*c) / (b*d)
//
// Отрицательный результат получается,
// если знаки исходных дробей различаются.
fn multiply_fractions(first: &Fraction, second: &Fraction) -> Fraction {
    let numerator = multiply_big(&first.numerator, &second.numerator);

    let denominator = multiply_big(&first.denominator, &second.denominator);

    let negative = first.negative != second.negative;

    let mut result = Fraction {
        numerator,
        denominator,
        negative,
    };

    reduce_fraction(&mut result);

    result
}

// --------------------------------------------------
// F) DIVIDE FRACTIONS
// --------------------------------------------------

// Делим первую дробь на вторую.
//
// a/b : c/d = (a*d) / (b*c)
//
// Деление на нулевую дробь запрещено.
fn divide_fractions(first: &Fraction, second: &Fraction) -> Fraction {
    assert!(
        is_zero(&second.numerator) == false,
        "Division by zero fraction"
    );

    // Числитель первой дроби умножаем
    // на знаменатель второй.
    let numerator = multiply_big(&first.numerator, &second.denominator);

    // Знаменатель первой дроби умножаем
    // на числитель второй.
    let denominator = multiply_big(&first.denominator, &second.numerator);

    let negative = first.negative != second.negative;

    let mut result = Fraction {
        numerator,
        denominator,
        negative,
    };

    reduce_fraction(&mut result);

    result
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    println!("BIG INTEGER FRACTIONS");
    println!("========================================");

    // A) Создаём дроби из строк преподавателя.
    let mut z1 = create_fraction("-56890/1223366");
    let mut z2 = create_fraction("2565689841/2365");

    // B) Выводим исходные дроби.
    println!("ORIGINAL FRACTIONS");

    print!("Z1 = ");
    print_fraction(&z1);

    print!("Z2 = ");
    print_fraction(&z2);

    // C) Сокращаем дроби до основного вида.
    reduce_fraction(&mut z1);
    reduce_fraction(&mut z2);

    println!();
    println!("REDUCED FRACTIONS");

    print!("Z1 = ");
    print_fraction(&z1);

    print!("Z2 = ");
    print_fraction(&z2);

    // D) Складываем две дроби.
    let z3 = add_fractions(&z1, &z2);

    println!();
    println!("D) ADDITION");

    print!("Z1 + Z2 = ");
    print_fraction(&z3);

    // E) Умножаем дроби.
    let z4 = multiply_fractions(&z1, &z2);

    println!();
    println!("E) MULTIPLICATION");

    print!("Z1 * Z2 = ");
    print_fraction(&z4);

    // F) Делим вторую дробь на первую.
    let z5 = divide_fractions(&z2, &z1);

    println!();
    println!("F) DIVISION");

    print!("Z2 / Z1 = ");
    print_fraction(&z5);

    // Дополнительные проверки.
    println!();
    println!("ADDITIONAL TESTS");

    let mut a = create_fraction("12/18");

    print!("Original: ");
    print_fraction(&a);

    reduce_fraction(&mut a);

    print!("Reduced:  ");
    print_fraction(&a);

    let b = create_fraction("-1/3");
    let c = create_fraction("1/3");

    print!("-1/3 + 1/3 = ");
    print_fraction(&add_fractions(&b, &c));

    let d = create_fraction("2/3");
    let e = create_fraction("3/4");

    print!("2/3 * 3/4 = ");
    print_fraction(&multiply_fractions(&d, &e));

    print!("2/3 / 3/4 = ");
    print_fraction(&divide_fractions(&d, &e));
}
