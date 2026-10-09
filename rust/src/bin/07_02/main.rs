/*
Cvičení 07 — Úloha 7.2
Název: Dlouhá reálná čísla
Body: 4 body; 1 bod; 1 bod; 5 bodů; 4 body; 3 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s dlouhými reálnými čísly v plovoucí řádové čárce, které používají způsob
uložení cifer v poli little endian (tento způsob byl použit na přednášce).

a) Funkci Vytvor, která převede textový řetězec na číslo. Textový řetězec může (ale nemusí)
obsahovat znak pro exponent. Př.: 7896e4 = 78960000; 5636.7e-2 =56.367; [4 body]

b) Funkci Vypis, která vypíše číslo na obrazovku bez zbytečných nul. [1 bod]

c) Funkce Compare porovná dvě čísla. Je-li první číslo menší než druhé číslo, je vrácena hodnota
-1. Je-li první číslo větší než druhé číslo, je vrácena hodnota 1. Jsou-li si obě čísla rovna, je
vrácena hodnota 0. [1 bod]

d) Funkce Soucet sečte dvě čísla. V závislosti na znaménku sčítaných čísel bude docházet ke sčítání
(př. 2 + 3) nebo odečítání (př. 2 + -3) absolutních hodnot čísel. Na rozdíl od implementace celých
čísel vyřešte i možná přetečení a podtečení výsledku. [5 bodů]

e) Funkce Soucin vynásobí dvě čísla. [4 body]

f) Funkce Podil vydělí číslo číslem. [3 body]


Ukázka:

Real r1 = Vytvor("-256.2e-1"); // - 2.562e1

Real r2 = Vytvor("-9534.07256e15"); // -9.53407256e18

Real r3 = Vytvor("9534.07256e15"); // 9.53407256e18

Vypis(r1); Vypis(r2); Vypis(r3);

int i = Compare(r1,r2); // -1

Real r4 = Soucet(r2, r2);  // -1.906814512e19

Real r5 = Soucet(r1, r3);  // 9.53407255999999997438e18

Real r6 = Soucet(r2, r3);  // 0

Real r7 = Soucin(r1, r2);  // 2.442629389872e20

Real r8 = Podil(r2, r1);  // 3.7213397970335675253...e17

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

const PRECISION: usize = 40;
const WORK_SIZE: usize = 100;
const PRODUCT_SIZE: usize = PRECISION * 2 + 1;

// Представление вещественного числа.
//
// digits — мантисса в Little Endian.
// negative — знак числа.
// exponent — степень десятки.
//
// Например:
// digits = [2, 6, 5, 2]
// exponent = -2
// negative = true
//
// Получаем -2562 * 10^-2 = -25.62.
#[derive(Clone, Copy)]
struct Real {
    digits: [u8; PRECISION],
    negative: bool,
    exponent: i64,
}

// Создаём нулевое число.
fn zero() -> Real {
    Real {
        digits: [0; PRECISION],
        negative: false,
        exponent: 0,
    }
}

// Проверяем, равно ли число нулю.
fn is_zero(number: &Real) -> bool {
    let mut i = 0;

    while i < PRECISION {
        if number.digits[i] != 0 {
            return false;
        }

        i += 1;
    }

    true
}

// Находим индекс самой старшей ненулевой цифры.
//
// Поскольку массив Little Endian,
// старшая цифра находится справа.
fn highest_digit(number: &Real) -> i32 {
    let mut i = PRECISION;

    while i > 0 {
        i -= 1;

        if number.digits[i] != 0 {
            return i as i32;
        }
    }

    -1
}

// Убираем младшие нули из мантиссы.
//
// Например:
// 25620 * 10^-3 = 2562 * 10^-2.
//
// Значение числа при этом не изменяется.
fn normalize(number: &mut Real) {
    if is_zero(number) {
        *number = zero();
        return;
    }

    let mut shift = 0;

    // Считаем нули в начале Little Endian массива.
    while shift < PRECISION && number.digits[shift] == 0 {
        shift += 1;
    }

    if shift > 0 {
        let mut i = 0;

        // Сдвигаем значащие цифры в начало.
        while i + shift < PRECISION {
            number.digits[i] = number.digits[i + shift];
            i += 1;
        }

        // Оставшиеся позиции заполняем нулями.
        while i < PRECISION {
            number.digits[i] = 0;
            i += 1;
        }

        // Компенсируем сдвиг изменением экспоненты.
        number.exponent += shift as i64;
    }
}

// --------------------------------------------------
// A) CREATE NUMBER
// --------------------------------------------------

// Преобразуем строку в Real.
//
// Поддерживаем:
// 123
// -123.45
// 7896e4
// 5636.7e-2
// -9534.07256e15
//
// Сохраняем до 40 значащих цифр.
fn create_number(text: &str) -> Real {
    let bytes = text.as_bytes();

    assert!(bytes.len() > 0, "Empty number");

    let mut result = zero();
    let mut i = 0;

    // Читаем необязательный знак.
    if bytes[i] == b'-' {
        result.negative = true;
        i += 1;
    } else if bytes[i] == b'+' {
        i += 1;
    }

    let mut after_point = false;
    let mut point_found = false;

    // Количество цифр после десятичной точки.
    let mut decimal_places: i64 = 0;

    // Общее количество значащих цифр.
    let mut significant_count: usize = 0;

    // Количество сохранённых цифр.
    let mut stored_count: usize = 0;

    // Временно храним цифры в прямом порядке.
    let mut stored = [0u8; PRECISION];

    let mut started = false;
    let mut total_digits = 0;

    // Читаем мантиссу.
    while i < bytes.len() {
        let character = bytes[i];

        // Начало экспоненты.
        if character == b'e' || character == b'E' {
            break;
        }

        if character == b'.' {
            assert!(point_found == false, "Invalid decimal point");

            point_found = true;
            after_point = true;
        } else {
            // Разрешаем только десятичные цифры.
            assert!(character >= b'0' && character <= b'9', "Invalid digit");

            total_digits += 1;

            if after_point {
                decimal_places += 1;
            }

            let digit = character - b'0';

            // Игнорируем ведущие нули.
            if digit != 0 {
                started = true;
            }

            if started {
                significant_count += 1;

                // Сохраняем только первые 40 значащих цифр.
                if stored_count < PRECISION {
                    stored[stored_count] = digit;
                    stored_count += 1;
                }
            }
        }

        i += 1;
    }

    assert!(total_digits > 0, "Number contains no digits");

    // Читаем экспоненту, если она присутствует.
    let mut scientific_exponent: i64 = 0;

    if i < bytes.len() {
        i += 1;

        assert!(i < bytes.len(), "Missing exponent");

        scientific_exponent = text[i..].parse::<i64>().expect("Invalid exponent");
    }

    // Если все цифры нулевые.
    if started == false {
        return zero();
    }

    // Превращаем сохранённые цифры в Little Endian.
    let mut j = 0;

    while j < stored_count {
        result.digits[j] = stored[stored_count - 1 - j];
        j += 1;
    }

    // Корректируем степень с учётом:
    // - цифр после точки;
    // - научной записи;
    // - отброшенных младших цифр.
    let removed_digits = significant_count - stored_count;

    result.exponent = scientific_exponent - decimal_places + removed_digits as i64;

    normalize(&mut result);

    result
}

// --------------------------------------------------
// B) PRINT NUMBER
// --------------------------------------------------

// Выводим число в научной записи.
//
// Убираем лишние нули с помощью normalize().
// Не преобразуем число в f64.
fn print_number(number: &Real) {
    if is_zero(number) {
        println!("0");
        return;
    }

    let highest = highest_digit(number) as usize;

    // Выводим минус для отрицательного числа.
    if number.negative {
        print!("-");
    }

    // Выводим старшую цифру.
    print!("{}", number.digits[highest]);

    // Остальные цифры выводим после точки.
    if highest > 0 {
        print!(".");

        let mut i = highest;

        while i > 0 {
            i -= 1;
            print!("{}", number.digits[i]);
        }
    }

    // Вычисляем степень в научной записи.
    let scientific_exponent = number.exponent + highest as i64;

    println!("e{}", scientific_exponent);
}

// --------------------------------------------------
// C) COMPARE
// --------------------------------------------------

// Возвращаем цифру числа в разряде 10^power.
//
// Если такого разряда нет, возвращаем ноль.
fn digit_at(number: &Real, power: i64) -> u8 {
    // i128 используется только для безопасного расчёта
    // индекса при очень больших экспонентах.
    let index = power as i128 - number.exponent as i128;

    if index < 0 || index >= PRECISION as i128 {
        return 0;
    }

    number.digits[index as usize]
}

// Сравниваем модули двух чисел.
//
// -1: |first| < |second|
//  0: |first| == |second|
//  1: |first| > |second|
fn compare_absolute(first: &Real, second: &Real) -> i32 {
    if is_zero(first) && is_zero(second) {
        return 0;
    }

    if is_zero(first) {
        return -1;
    }

    if is_zero(second) {
        return 1;
    }

    // Определяем положение старшей цифры.
    let first_power = first.exponent + highest_digit(first) as i64;
    let second_power = second.exponent + highest_digit(second) as i64;

    // Сначала сравниваем порядки чисел.
    if first_power < second_power {
        return -1;
    }

    if first_power > second_power {
        return 1;
    }

    // Если порядки равны, сравниваем цифры.
    let mut i = 0;

    while i < PRECISION {
        let power = first_power - i as i64;

        let a = digit_at(first, power);
        let b = digit_at(second, power);

        if a < b {
            return -1;
        }

        if a > b {
            return 1;
        }

        i += 1;
    }

    0
}

// Сравниваем числа с учётом знака.
fn compare(first: &Real, second: &Real) -> i32 {
    // Отрицательное всегда меньше положительного.
    if first.negative && second.negative == false {
        return -1;
    }

    if first.negative == false && second.negative {
        return 1;
    }

    let result = compare_absolute(first, second);

    // Для двух отрицательных чисел порядок обратный.
    if first.negative {
        return -result;
    }

    result
}

// --------------------------------------------------
// HELPERS FOR ADDITION
// --------------------------------------------------

// Преобразуем временный массив цифр в Real.
//
// work тоже хранит цифры в Little Endian.
//
// Если результат длиннее 40 цифр,
// отбрасываем младшие разряды.
fn from_work(work: &[u8], base_exponent: i64, negative: bool) -> Real {
    let mut result = zero();

    // Ищем старшую ненулевую цифру.
    let mut highest = work.len();

    while highest > 0 {
        highest -= 1;

        if work[highest] != 0 {
            break;
        }
    }

    if work[highest] == 0 {
        return result;
    }

    // Вычисляем количество отбрасываемых цифр.
    let shift = if highest + 1 > PRECISION {
        highest + 1 - PRECISION
    } else {
        0
    };

    result.negative = negative;
    result.exponent = base_exponent + shift as i64;

    // Копируем оставшиеся значащие цифры.
    let mut i = 0;

    while i < PRECISION && i + shift < work.len() {
        result.digits[i] = work[i + shift];
        i += 1;
    }

    normalize(&mut result);

    result
}

// Складываем абсолютные значения.
//
// Выравниваем числа по десятичным разрядам,
// затем складываем цифры с переносом.
fn add_absolute(first: &Real, second: &Real) -> Real {
    let first_top = first.exponent + highest_digit(first) as i64;
    let second_top = second.exponent + highest_digit(second) as i64;

    let top = if first_top > second_top {
        first_top
    } else {
        second_top
    };

    // Оставляем место под перенос и младшие цифры.
    let base = top - (WORK_SIZE as i64 - 3);

    let mut work = [0u8; WORK_SIZE];
    let mut carry: u16 = 0;

    let mut i = 0;

    while i < WORK_SIZE {
        let power = base + i as i64;

        let sum = digit_at(first, power) as u16 + digit_at(second, power) as u16 + carry;

        work[i] = (sum % 10) as u8;
        carry = sum / 10;

        i += 1;
    }

    from_work(&work, base, false)
}

// Вычитаем абсолютные значения.
//
// Предполагается, что |first| >= |second|.
fn subtract_absolute(first: &Real, second: &Real) -> Real {
    let first_top = first.exponent + highest_digit(first) as i64;

    let base = first_top - (WORK_SIZE as i64 - 3);

    let mut work = [0u8; WORK_SIZE];
    let mut borrow = 0;

    let mut i = 0;

    while i < WORK_SIZE {
        let power = base + i as i64;

        let mut difference =
            digit_at(first, power) as i32 - digit_at(second, power) as i32 - borrow;

        // Если не хватает цифры, занимаем десятку.
        if difference < 0 {
            difference += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        work[i] = difference as u8;

        i += 1;
    }

    from_work(&work, base, false)
}

// --------------------------------------------------
// D) ADDITION
// --------------------------------------------------

// Складываем два вещественных числа.
//
// При одинаковых знаках складываем модули.
// При разных знаках вычитаем меньший модуль
// из большего и выбираем соответствующий знак.
fn add(first: &Real, second: &Real) -> Real {
    if is_zero(first) {
        return *second;
    }

    if is_zero(second) {
        return *first;
    }

    // Одинаковые знаки.
    if first.negative == second.negative {
        let mut result = add_absolute(first, second);
        result.negative = first.negative;

        return result;
    }

    // Разные знаки.
    let comparison = compare_absolute(first, second);

    // Модули равны: сумма равна нулю.
    if comparison == 0 {
        return zero();
    }

    if comparison > 0 {
        let mut result = subtract_absolute(first, second);
        result.negative = first.negative;

        return result;
    }

    let mut result = subtract_absolute(second, first);
    result.negative = second.negative;

    result
}

// --------------------------------------------------
// E) MULTIPLICATION
// --------------------------------------------------

// Умножаем числа столбиком.
//
// Каждую цифру первого числа умножаем
// на каждую цифру второго.
//
// После этого переносим десятки.
fn multiply(first: &Real, second: &Real) -> Real {
    if is_zero(first) || is_zero(second) {
        return zero();
    }

    // Временный массив для произведения.
    let mut work = [0u32; PRODUCT_SIZE];

    let mut i = 0;

    while i < PRECISION {
        let mut j = 0;

        while j < PRECISION {
            work[i + j] += first.digits[i] as u32 * second.digits[j] as u32;

            j += 1;
        }

        i += 1;
    }

    // Переносим десятки по разрядам.
    i = 0;

    while i + 1 < PRODUCT_SIZE {
        let carry = work[i] / 10;

        work[i] %= 10;
        work[i + 1] += carry;

        i += 1;
    }

    // Переносим цифры в обычный массив u8.
    let mut digits = [0u8; PRODUCT_SIZE];

    i = 0;

    while i < PRODUCT_SIZE {
        digits[i] = work[i] as u8;
        i += 1;
    }

    // При умножении степени складываются.
    let exponent = first.exponent + second.exponent;

    // Результат отрицателен, если знаки различаются.
    let negative = first.negative != second.negative;

    from_work(&digits, exponent, negative)
}

// --------------------------------------------------
// HELPERS FOR DIVISION
// --------------------------------------------------

// Сравниваем два массива цифр Little Endian.
//
// Массивы должны иметь одинаковую длину.
fn compare_digits(first: &[u8], second: &[u8]) -> i32 {
    assert!(first.len() == second.len());

    let mut i = first.len();

    while i > 0 {
        i -= 1;

        if first[i] < second[i] {
            return -1;
        }

        if first[i] > second[i] {
            return 1;
        }
    }

    0
}

// Вычитаем second из first.
//
// Результат записываем непосредственно в first.
// Предполагаем, что first >= second.
fn subtract_digits(first: &mut [u8], second: &[u8]) {
    let mut borrow = 0;
    let mut i = 0;

    while i < first.len() {
        let mut difference = first[i] as i32 - second[i] as i32 - borrow;

        if difference < 0 {
            difference += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        first[i] = difference as u8;

        i += 1;
    }
}

// --------------------------------------------------
// F) DIVISION
// --------------------------------------------------

// Делим первое число на второе столбиком.
//
// Находим 40 значащих цифр частного.
// Деление на ноль запрещено.
//
// После получения 40 цифр младшие разряды
// результата отбрасываются.
fn divide(first: &Real, second: &Real) -> Real {
    assert!(is_zero(second) == false, "Division by zero");

    if is_zero(first) {
        return zero();
    }

    // Остаток и делитель храним в массивах
    // с дополнительным старшим разрядом.
    let mut remainder = [0u8; PRECISION + 1];
    let mut divisor = [0u8; PRECISION + 1];

    let mut i = 0;

    while i < PRECISION {
        divisor[i] = second.digits[i];
        i += 1;
    }

    // Цифры частного временно записываются
    // слева направо.
    let mut quotient = [0u8; PRECISION];

    let mut written = 0;
    let mut started = false;
    let mut first_power = 0i64;

    // Начинаем со старшей значащей цифры делимого.
    let mut position = highest_digit(first) as i64;

    while written < PRECISION {
        // Умножаем остаток на 10.
        // Для Little Endian сдвигаем цифры вправо.
        let mut j = PRECISION;

        while j > 0 {
            remainder[j] = remainder[j - 1];
            j -= 1;
        }

        remainder[0] = 0;

        // Сносим следующую цифру делимого.
        // Когда они закончатся, сносим нули.
        if position >= 0 {
            remainder[0] = first.digits[position as usize];
        }

        // Вычисляем очередную цифру частного.
        let mut digit = 0u8;

        while compare_digits(&remainder, &divisor) >= 0 {
            subtract_digits(&mut remainder, &divisor);
            digit += 1;
        }

        // Пропускаем ведущие нули частного.
        if digit != 0 && started == false {
            started = true;
            first_power = position;
        }

        if started {
            quotient[written] = digit;
            written += 1;
        }

        // Переходим к следующему десятичному разряду.
        position -= 1;
    }

    // Создаём итоговое число.
    let mut result = zero();

    // Переворачиваем цифры в Little Endian.
    i = 0;

    while i < PRECISION {
        result.digits[i] = quotient[PRECISION - 1 - i];
        i += 1;
    }

    // При делении степени вычитаются.
    //
    // Дополнительно учитываем положение
    // первой значащей цифры частного.
    result.exponent = first.exponent - second.exponent + first_power - (PRECISION as i64 - 1);

    result.negative = first.negative != second.negative;

    normalize(&mut result);

    result
}

// --------------------------------------------------
// MAIN
// --------------------------------------------------

fn main() {
    // Данные из задания преподавателя.
    let r1 = create_number("-256.2e-1");
    let r2 = create_number("-9534.07256e15");
    let r3 = create_number("9534.07256e15");

    println!("LONG REAL NUMBER OPERATIONS");
    println!("========================================");

    // A + B) Создание и вывод чисел.
    println!("INPUT NUMBERS");

    print!("R1: ");
    print_number(&r1);

    print!("R2: ");
    print_number(&r2);

    print!("R3: ");
    print_number(&r3);

    // C) Сравнение.
    println!();
    println!("C) COMPARISON");

    println!("R1 vs R2: {}", compare(&r1, &r2));
    println!("R2 vs R3: {}", compare(&r2, &r3));

    // D) Сложение.
    println!();
    println!("D) ADDITION");

    let r4 = add(&r2, &r2);

    print!("R2 + R2 = ");
    print_number(&r4);

    let r5 = add(&r1, &r3);

    print!("R1 + R3 = ");
    print_number(&r5);

    let r6 = add(&r2, &r3);

    print!("R2 + R3 = ");
    print_number(&r6);

    // E) Умножение.
    println!();
    println!("E) MULTIPLICATION");

    let r7 = multiply(&r1, &r2);

    print!("R1 * R2 = ");
    print_number(&r7);

    // F) Деление.
    println!();
    println!("F) DIVISION");

    let r8 = divide(&r2, &r1);

    print!("R2 / R1 = ");
    print_number(&r8);

    // Дополнительные проверки.
    println!();
    println!("ADDITIONAL TESTS");

    let a = create_number("12.5");
    let b = create_number("2.5");

    print!("12.5 + 2.5 = ");
    print_number(&add(&a, &b));

    print!("12.5 * 2.5 = ");
    print_number(&multiply(&a, &b));

    print!("12.5 / 2.5 = ");
    print_number(&divide(&a, &b));

    let one = create_number("1");
    let three = create_number("3");

    print!("1 / 3 = ");
    print_number(&divide(&one, &three));

    let example1 = create_number("7896e4");
    let example2 = create_number("5636.7e-2");

    print!("7896e4 = ");
    print_number(&example1);

    print!("5636.7e-2 = ");
    print_number(&example2);
}
