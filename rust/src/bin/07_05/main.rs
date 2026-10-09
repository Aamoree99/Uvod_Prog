/*
Cvičení 07 — Úloha 7.5
Název: Dlouhá celá čísla v obecné číselné soustavě
Body: 1/2 bodu; 1/2 bodu; 1/2 bodu; 1/2 bodu; 3 body (v pořadí podúloh)

Zadání:
Funkce z příkladu 7.1 upravte tak, aby pracovaly s čísly v zadané číselné soustavě:

a) Funkce Soucet sečte dvě čísla. [1/2 bodu]

b) Funkce Rozdil odečte menší číslo od většího. [1/2 bodu]

c) Funkce Soucin vynásobí dvě čísla. [1/2 bodu]

d) Funkce Podil vydělí číslo číslem. [1/2 bodu]

e) Napište funkci Prevod, která převede číslo používající způsob uložení cifer big endian z jedné
číselné soustavy do druhé číselné soustavy. Při praktických pokusech používejte číselné soustavy o
maximálním základu 10, snáze se v nich zobrazují výsledky. [3 body]


Ukázka:

int[] c1 = Vytvor("560111202232646");

int[] c2 = Prevod(c1, 10, 2);

// 1111111010110101100001101001111100110100101000110

int[] c3 = Prevod(c2, 2, 5); // 1041403330220232421041

int[] c4 = Soucet(c3, c3, 5); // 2133312210441020342132

int[] c5 = Rozdil(c4, c3, 5); // 1041403330220232421041

int[] c6 = Prevod(c5, 5, 7); // 225656461511451251

int[] c7 = Soucin(c6, c6, 7);

// 55402656051563425266136012062265231

int[] c8 = Podil(c7, c6, 7); // 225656461511451251

int[] c9 = Prevod(c8, 7, 10); // 560111202232646

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

// Максимальное количество цифр.
//
// Используем 128, потому что двоичная запись
// занимает больше цифр, чем десятичная.
const MAX_DIGITS: usize = 128;

// Длинное число в формате Big Endian.
//
// Например, число 123:
// [0, 0, ... 0, 1, 2, 3]
type BigNumber = [u8; MAX_DIGITS];

// --------------------------------------------------
// BASIC FUNCTIONS
// --------------------------------------------------

// Проверяем основание системы счисления.
//
// В этом упражнении используем основания 2..=10.
fn check_base(base: u32) {
    assert!(base >= 2 && base <= 10, "Base must be between 2 and 10");
}

// Создаём нулевое число.
fn zero() -> BigNumber {
    [0; MAX_DIGITS]
}

// Проверяем, равно ли число нулю.
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

// Преобразуем строку в массив цифр.
//
// Эта функция читает числа с цифрами 0..9.
// Корректность цифр для конкретной системы
// проверяется отдельно.
//
// Например:
// "123" -> [0, 0, ... 0, 1, 2, 3]
fn create_number(text: &str) -> BigNumber {
    let bytes = text.as_bytes();

    assert!(bytes.len() > 0, "Empty number");
    assert!(bytes.len() <= MAX_DIGITS, "Number is too long");

    let mut result = zero();

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
fn print_number(number: &BigNumber) {
    let mut i = 0;

    // Пропускаем нули слева.
    while i + 1 < MAX_DIGITS && number[i] == 0 {
        i += 1;
    }

    while i < MAX_DIGITS {
        print!("{}", number[i]);
        i += 1;
    }

    println!();
}

// Сравниваем два длинных числа.
//
// -1 — первое меньше.
//  0 — числа равны.
//  1 — первое больше.
//
// Числа должны быть в одной системе счисления.
fn compare(first: &BigNumber, second: &BigNumber) -> i32 {
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

// Проверяем, допустимы ли все цифры числа
// для заданного основания.
//
// Например, в двоичной системе
// разрешены только 0 и 1.
fn validate_number(number: &BigNumber, base: u32) {
    check_base(base);

    let mut i = 0;

    while i < MAX_DIGITS {
        assert!(
            (number[i] as u32) < base,
            "Invalid digit for numeral system"
        );

        i += 1;
    }
}

// --------------------------------------------------
// A) ADDITION
// --------------------------------------------------

// Складываем два числа в заданной системе.
//
// Отличие от десятичного сложения:
//
// Вместо % 10 используем % base.
// Вместо / 10 используем / base.
//
// Например, в двоичной системе:
// 1 + 1 = 10.
fn add(first: &BigNumber, second: &BigNumber, base: u32) -> BigNumber {
    validate_number(first, base);
    validate_number(second, base);

    let mut result = zero();

    let mut carry: u32 = 0;
    let mut i = MAX_DIGITS;

    // Складываем цифры справа налево.
    while i > 0 {
        i -= 1;

        let sum = first[i] as u32 + second[i] as u32 + carry;

        // Записываем текущую цифру.
        result[i] = (sum % base) as u8;

        // Переносим остаток в следующий разряд.
        carry = sum / base;
    }

    assert!(carry == 0, "Addition overflow");

    result
}

// --------------------------------------------------
// B) SUBTRACTION
// --------------------------------------------------

// Вычитаем меньшее число из большего.
//
// Результат всегда неотрицательный.
//
// При заимствовании прибавляем base,
// а не обязательно 10.
fn subtract(first: &BigNumber, second: &BigNumber, base: u32) -> BigNumber {
    validate_number(first, base);
    validate_number(second, base);

    // Определяем большее и меньшее число.
    let (larger, smaller) = if compare(first, second) >= 0 {
        (first, second)
    } else {
        (second, first)
    };

    let mut result = zero();

    let mut borrow = 0i32;
    let mut i = MAX_DIGITS;

    while i > 0 {
        i -= 1;

        let mut difference = larger[i] as i32 - smaller[i] as i32 - borrow;

        // Если не хватает цифры,
        // занимаем единицу следующего разряда.
        if difference < 0 {
            difference += base as i32;
            borrow = 1;
        } else {
            borrow = 0;
        }

        result[i] = difference as u8;
    }

    result
}

// --------------------------------------------------
// C) MULTIPLICATION
// --------------------------------------------------

// Умножаем числа столбиком.
//
// Для промежуточного произведения
// используем массив двойной длины.
//
// Перенос выполняется по основанию base.
fn multiply(first: &BigNumber, second: &BigNumber, base: u32) -> BigNumber {
    validate_number(first, base);
    validate_number(second, base);

    let mut work = [0u32; MAX_DIGITS * 2];

    let mut i = MAX_DIGITS;

    // Берём каждую цифру первого числа.
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

    // Переносим разряды справа налево.
    let mut i = MAX_DIGITS * 2;

    while i > 1 {
        i -= 1;

        let carry = work[i] / base;

        work[i] %= base;
        work[i - 1] += carry;
    }

    assert!(work[0] < base, "Multiplication overflow");

    // Проверяем, что результат помещается
    // в MAX_DIGITS цифр.
    i = 0;

    while i < MAX_DIGITS {
        assert!(work[i] == 0, "Multiplication overflow");

        i += 1;
    }

    // Копируем младшую половину произведения.
    let mut result = zero();
    i = 0;

    while i < MAX_DIGITS {
        result[i] = work[MAX_DIGITS + i] as u8;
        i += 1;
    }

    result
}

// --------------------------------------------------
// HELPERS FOR DIVISION
// --------------------------------------------------

// Сравниваем два расширенных массива.
//
// Дополнительная ячейка используется
// для промежуточного остатка при делении.
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

// Вычитаем second из first.
//
// Результат записывается в first.
//
// Используем заданное основание base.
fn subtract_extended(first: &mut [u8; MAX_DIGITS + 1], second: &[u8; MAX_DIGITS + 1], base: u32) {
    let mut borrow = 0i32;
    let mut i = MAX_DIGITS + 1;

    while i > 0 {
        i -= 1;

        let mut difference = first[i] as i32 - second[i] as i32 - borrow;

        if difference < 0 {
            difference += base as i32;
            borrow = 1;
        } else {
            borrow = 0;
        }

        first[i] = difference as u8;
    }
}

// --------------------------------------------------
// D) DIVISION
// --------------------------------------------------

// Делим первое число на второе столбиком.
//
// Возвращаем:
// - целую часть частного;
// - остаток от деления.
//
// Каждая цифра частного меньше base.
fn divide(first: &BigNumber, second: &BigNumber, base: u32) -> (BigNumber, BigNumber) {
    validate_number(first, base);
    validate_number(second, base);

    assert!(is_zero(second) == false, "Division by zero");

    let mut quotient = zero();

    // Остаток с дополнительным старшим разрядом.
    let mut remainder = [0u8; MAX_DIGITS + 1];

    // Делитель с дополнительным нулём слева.
    let mut divisor = [0u8; MAX_DIGITS + 1];

    let mut i = 0;

    while i < MAX_DIGITS {
        divisor[i + 1] = second[i];
        i += 1;
    }

    // Проходим делимое слева направо.
    i = 0;

    while i < MAX_DIGITS {
        // Сдвигаем остаток влево на один разряд.
        // Это эквивалентно умножению на base.
        let mut j = 0;

        while j < MAX_DIGITS {
            remainder[j] = remainder[j + 1];
            j += 1;
        }

        // Сносим следующую цифру делимого.
        remainder[MAX_DIGITS] = first[i];

        // Определяем очередную цифру частного.
        let mut digit = 0u8;

        while compare_extended(&remainder, &divisor) >= 0 {
            subtract_extended(&mut remainder, &divisor, base);

            digit += 1;
        }

        // При правильном делении очередная
        // цифра частного всегда меньше основания.
        assert!((digit as u32) < base, "Invalid quotient digit");

        quotient[i] = digit;

        i += 1;
    }

    // Переносим остаток в обычный массив.
    let mut final_remainder = zero();
    i = 0;

    while i < MAX_DIGITS {
        final_remainder[i] = remainder[i + 1];
        i += 1;
    }

    (quotient, final_remainder)
}

// --------------------------------------------------
// E) BASE CONVERSION
// --------------------------------------------------

// Переводим длинное число между системами счисления.
//
// from_base — исходное основание.
// to_base — новое основание.
//
// Например:
// 1010 (base 2) -> 10 (base 10)
//
// Алгоритм:
// 1. Начинаем с нуля в целевой системе.
// 2. Читаем цифры исходного числа слева направо.
// 3. Умножаем результат на исходное основание.
// 4. Прибавляем очередную исходную цифру.
//
// Всё вычисление выполняем непосредственно
// над массивом в целевой системе счисления.
//
// Это позволяет не переводить большое число
// в i32, i64 или i128.
fn convert(number: &BigNumber, from_base: u32, to_base: u32) -> BigNumber {
    validate_number(number, from_base);
    check_base(to_base);

    let mut result = zero();

    // Перебираем цифры исходного числа
    // слева направо, поскольку оно Big Endian.
    let mut i = 0;

    while i < MAX_DIGITS {
        // Текущая исходная цифра добавляется
        // к младшему разряду результата.
        let mut carry = number[i] as u32;

        let mut j = MAX_DIGITS;

        // Выполняем:
        // result = result * from_base + number[i]
        //
        // Но цифры результата записываем
        // уже в новой системе to_base.
        while j > 0 {
            j -= 1;

            let value = result[j] as u32 * from_base + carry;

            result[j] = (value % to_base) as u8;
            carry = value / to_base;
        }

        // Проверяем, что число поместилось в массив.
        assert!(carry == 0, "Base conversion overflow");

        i += 1;
    }

    result
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    println!("BIG INTEGER NUMERAL SYSTEMS");
    println!("========================================");

    // Исходное десятичное число из задания.
    let c1 = create_number("560111202232646");

    print!("C1 (base 10): ");
    print_number(&c1);

    // Переводим из десятичной системы в двоичную.
    let c2 = convert(&c1, 10, 2);

    print!("C2 (base 2):  ");
    print_number(&c2);

    // Переводим из двоичной системы в пятеричную.
    let c3 = convert(&c2, 2, 5);

    print!("C3 (base 5):  ");
    print_number(&c3);

    // Складываем два пятеричных числа.
    let c4 = add(&c3, &c3, 5);

    print!("C4 (base 5):  ");
    print_number(&c4);

    // Вычитаем одно пятеричное число из другого.
    let c5 = subtract(&c4, &c3, 5);

    print!("C5 (base 5):  ");
    print_number(&c5);

    // Переводим из пятеричной системы в семеричную.
    let c6 = convert(&c5, 5, 7);

    print!("C6 (base 7):  ");
    print_number(&c6);

    // Возводим семеричное число в квадрат.
    let c7 = multiply(&c6, &c6, 7);

    print!("C7 (base 7):  ");
    print_number(&c7);

    // Делим произведение на исходный множитель.
    let (c8, remainder) = divide(&c7, &c6, 7);

    print!("C8 (base 7):  ");
    print_number(&c8);

    // Переводим результат обратно в десятичную систему.
    let c9 = convert(&c8, 7, 10);

    print!("C9 (base 10): ");
    print_number(&c9);

    // Проверяем, что деление прошло без остатка.
    assert!(is_zero(&remainder), "Unexpected division remainder");

    // Проверяем, что получили исходное число.
    assert!(compare(&c1, &c9) == 0, "Round-trip conversion failed");

    println!();
    println!("ALL CHECKS PASSED");

    // Дополнительные короткие тесты.
    println!();
    println!("ADDITIONAL TESTS");

    let binary_a = create_number("1010");
    let binary_b = create_number("11");

    print!("1010 + 11 (base 2) = ");
    print_number(&add(&binary_a, &binary_b, 2));

    print!("1010 - 11 (base 2) = ");
    print_number(&subtract(&binary_a, &binary_b, 2));

    print!("1010 * 11 (base 2) = ");
    print_number(&multiply(&binary_a, &binary_b, 2));

    let (quotient, remainder) = divide(&binary_a, &binary_b, 2);

    print!("1010 / 11 (base 2) = ");
    print_number(&quotient);

    print!("Remainder (base 2) = ");
    print_number(&remainder);

    print!("1010 (base 2) -> base 10 = ");
    print_number(&convert(&binary_a, 2, 10));
}
