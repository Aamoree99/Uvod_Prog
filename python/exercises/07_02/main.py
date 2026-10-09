"""
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
"""

PRECISION = 40
WORK_SIZE = 100

# Число: (массив цифр от младших к старшим, отрицательный знак, показатель).
# Встроенные float/Decimal для длинной арифметики не используются.
def zero():
    return [0] * PRECISION, False, 0


def is_zero(number):
    i = 0
    while i < PRECISION:
        if number[0][i] != 0:
            return False
        i += 1
    return True


def highest_digit(number):
    i = PRECISION - 1
    while i >= 0:
        if number[0][i] != 0:
            return i
        i -= 1
    return -1


def normalize(number):
    if is_zero(number):
        return zero()
    digits, negative, exponent = number
    shift = 0
    while shift < PRECISION and digits[shift] == 0:
        shift += 1
    result = [0] * PRECISION
    i = 0
    while i + shift < PRECISION:
        result[i] = digits[i + shift]
        i += 1
    return result, negative, exponent + shift


def create_number(text):
    if len(text) == 0:
        raise ValueError("Empty number")
    negative = text[0] == '-'
    i = 1 if text[0] in ('-', '+') else 0
    point_found = started = False
    decimal_places = significant_count = stored_count = total_digits = 0
    stored = [0] * PRECISION
    while i < len(text) and text[i] not in ('e', 'E'):
        character = text[i]
        if character == '.':
            if point_found:
                raise ValueError("Invalid decimal point")
            point_found = True
        else:
            if not '0' <= character <= '9':
                raise ValueError("Invalid digit")
            total_digits += 1
            if point_found:
                decimal_places += 1
            digit = ord(character) - ord('0')
            if digit != 0:
                started = True
            if started:
                significant_count += 1
                if stored_count < PRECISION:
                    stored[stored_count] = digit
                    stored_count += 1
        i += 1
    if total_digits == 0:
        raise ValueError("Number contains no digits")
    scientific_exponent = 0
    if i < len(text):
        # Показатель машинный; мантисса остаётся массивом цифр.
        exponent_text = text[i + 1:]
        j = 1 if exponent_text[:1] in ('-', '+') else 0
        if j == len(exponent_text):
            raise ValueError("Missing exponent")
        while j < len(exponent_text):
            if not '0' <= exponent_text[j] <= '9':
                raise ValueError("Invalid exponent")
            j += 1
        scientific_exponent = int(exponent_text)
    if not started:
        return zero()
    digits = [0] * PRECISION
    j = 0
    while j < stored_count:
        digits[j] = stored[stored_count - 1 - j]
        j += 1
    exponent = scientific_exponent - decimal_places + significant_count - stored_count
    return normalize((digits, negative, exponent))


def number_text(number):
    if is_zero(number):
        return "0"
    digits, negative, exponent = number
    highest = highest_digit(number)
    result = ('-' if negative else '') + str(digits[highest])
    if highest > 0:
        result += '.'
        i = highest - 1
        while i >= 0:
            result += str(digits[i])
            i -= 1
    return result + 'e' + str(exponent + highest)


def print_number(number):
    print(number_text(number))


def digit_at(number, power):
    index = power - number[2]
    return number[0][index] if 0 <= index < PRECISION else 0


def compare_absolute(first, second):
    if is_zero(first):
        return 0 if is_zero(second) else -1
    if is_zero(second):
        return 1
    first_top = first[2] + highest_digit(first)
    second_top = second[2] + highest_digit(second)
    if first_top != second_top:
        return -1 if first_top < second_top else 1
    i = 0
    while i < PRECISION:
        a, b = digit_at(first, first_top - i), digit_at(second, first_top - i)
        if a != b:
            return -1 if a < b else 1
        i += 1
    return 0


def compare(first, second):
    if first[1] != second[1]:
        return -1 if first[1] else 1
    result = compare_absolute(first, second)
    return -result if first[1] else result


def from_work(work, base_exponent, negative):
    highest = len(work) - 1
    while highest > 0 and work[highest] == 0:
        highest -= 1
    if work[highest] == 0:
        return zero()
    shift = highest + 1 - PRECISION if highest + 1 > PRECISION else 0
    digits = [0] * PRECISION
    i = 0
    while i < PRECISION and i + shift < len(work):
        digits[i] = work[i + shift]
        i += 1
    return normalize((digits, negative, base_exponent + shift))


def add_absolute(first, second):
    first_top = first[2] + highest_digit(first)
    second_top = second[2] + highest_digit(second)
    top = first_top if first_top > second_top else second_top
    base = top - (WORK_SIZE - 3)
    work = [0] * WORK_SIZE
    carry = i = 0
    while i < WORK_SIZE:
        total = digit_at(first, base + i) + digit_at(second, base + i) + carry
        work[i], carry = total % 10, total // 10
        i += 1
    return from_work(work, base, False)


def subtract_absolute(first, second):
    base = first[2] + highest_digit(first) - (WORK_SIZE - 3)
    work = [0] * WORK_SIZE
    # Ненулевые отброшенные младшие цифры меньшего числа дают заём.
    # Без него 1e100 + (-1) ошибочно превращается в 1e100.
    borrow = 0
    j = 0
    while j < PRECISION and second[2] + j < base:
        if second[0][j] != 0:
            borrow = 1
            break
        j += 1
    i = 0
    while i < WORK_SIZE:
        difference = digit_at(first, base + i) - digit_at(second, base + i) - borrow
        if difference < 0:
            difference += 10
            borrow = 1
        else:
            borrow = 0
        work[i] = difference
        i += 1
    return from_work(work, base, False)


def add(first, second):
    if is_zero(first):
        return second[0].copy(), second[1], second[2]
    if is_zero(second):
        return first[0].copy(), first[1], first[2]
    if first[1] == second[1]:
        result = add_absolute(first, second)
        return result[0], first[1], result[2]
    comparison = compare_absolute(first, second)
    if comparison == 0:
        return zero()
    if comparison > 0:
        result, negative = subtract_absolute(first, second), first[1]
    else:
        result, negative = subtract_absolute(second, first), second[1]
    return result[0], negative, result[2]


def multiply(first, second):
    if is_zero(first) or is_zero(second):
        return zero()
    work = [0] * (PRECISION * 2 + 1)
    i = 0
    while i < PRECISION:
        j = 0
        while j < PRECISION:
            work[i + j] += first[0][i] * second[0][j]
            j += 1
        i += 1
    i = 0
    while i + 1 < len(work):
        work[i + 1] += work[i] // 10
        work[i] %= 10
        i += 1
    return from_work(work, first[2] + second[2], first[1] != second[1])


def compare_digits(first, second):
    i = len(first) - 1
    while i >= 0:
        if first[i] != second[i]:
            return -1 if first[i] < second[i] else 1
        i -= 1
    return 0


def subtract_digits(first, second):
    borrow = i = 0
    while i < len(first):
        difference = first[i] - second[i] - borrow
        if difference < 0:
            difference += 10
            borrow = 1
        else:
            borrow = 0
        first[i] = difference
        i += 1


def divide(first, second):
    if is_zero(second):
        raise ZeroDivisionError("Division by zero")
    if is_zero(first):
        return zero()
    remainder = [0] * (PRECISION + 1)
    divisor = [0] * (PRECISION + 1)
    i = 0
    while i < PRECISION:
        divisor[i] = second[0][i]
        i += 1
    quotient = [0] * PRECISION
    written = first_power = 0
    started = False
    position = highest_digit(first)
    while written < PRECISION:
        j = PRECISION
        while j > 0:
            remainder[j] = remainder[j - 1]
            j -= 1
        remainder[0] = first[0][position] if position >= 0 else 0
        digit = 0
        while compare_digits(remainder, divisor) >= 0:
            subtract_digits(remainder, divisor)
            digit += 1
        if digit != 0 and not started:
            started = True
            first_power = position
        if started:
            quotient[written] = digit
            written += 1
        position -= 1
    digits = [0] * PRECISION
    i = 0
    while i < PRECISION:
        digits[i] = quotient[PRECISION - 1 - i]
        i += 1
    exponent = first[2] - second[2] + first_power - (PRECISION - 1)
    return normalize((digits, first[1] != second[1], exponent))


def main():
    r1 = create_number('-256.2e-1')
    r2 = create_number('-9534.07256e15')
    r3 = create_number('9534.07256e15')
    print(f"R1: {number_text(r1)}\nR2: {number_text(r2)}\nR3: {number_text(r3)}")
    print(f"R1 vs R2: {compare(r1, r2)}\nR2 vs R3: {compare(r2, r3)}")
    print(f"R2 + R2 = {number_text(add(r2, r2))}")
    print(f"R1 + R3 = {number_text(add(r1, r3))}")
    print(f"R2 + R3 = {number_text(add(r2, r3))}")
    print(f"R1 * R2 = {number_text(multiply(r1, r2))}")
    print(f"R2 / R1 = {number_text(divide(r2, r1))}")
    a, b = create_number('12.5'), create_number('2.5')
    print(f"12.5 + 2.5 = {number_text(add(a, b))}")
    print(f"12.5 * 2.5 = {number_text(multiply(a, b))}")
    print(f"12.5 / 2.5 = {number_text(divide(a, b))}")
    print(f"1 / 3 = {number_text(divide(create_number('1'), create_number('3')))}")
    print(f"7896e4 = {number_text(create_number('7896e4'))}")
    print(f"5636.7e-2 = {number_text(create_number('5636.7e-2'))}")

if __name__ == "__main__":
    main()
