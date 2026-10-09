"""
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
"""

MAX_DIGITS = 128

def check_base(base):
    if base < 2 or base > 10:
        raise ValueError("Base must be between 2 and 10")


def zero():
    return [0] * MAX_DIGITS


def is_zero(number):
    i = 0
    while i < len(number):
        if number[i] != 0:
            return False
        i += 1
    return True


def create_number(text):
    if len(text) == 0 or len(text) > MAX_DIGITS:
        raise ValueError("Invalid number length")
    result = zero()
    offset = MAX_DIGITS - len(text)
    i = 0
    while i < len(text):
        if not '0' <= text[i] <= '9':
            raise ValueError("Invalid digit")
        result[offset + i] = ord(text[i]) - ord('0')
        i += 1
    return result


def number_text(number):
    i = 0
    while i + 1 < len(number) and number[i] == 0:
        i += 1
    text = ""
    while i < len(number):
        text += chr(ord('0') + number[i])
        i += 1
    return text


def print_number(number):
    print(number_text(number))


def compare(first, second):
    if len(first) != len(second):
        raise ValueError("Different array lengths")
    i = 0
    while i < len(first):
        if first[i] < second[i]:
            return -1
        if first[i] > second[i]:
            return 1
        i += 1
    return 0


def validate_number(number, base):
    check_base(base)
    if len(number) != MAX_DIGITS:
        raise ValueError("Invalid digit array length")
    i = 0
    while i < MAX_DIGITS:
        if number[i] < 0 or number[i] >= base:
            raise ValueError("Invalid digit for numeral system")
        i += 1


def add(first, second, base=10):
    validate_number(first, base)
    validate_number(second, base)
    result = zero()
    carry = 0
    i = MAX_DIGITS - 1
    while i >= 0:
        total = first[i] + second[i] + carry
        result[i] = total % base
        carry = total // base
        i -= 1
    if carry != 0:
        raise OverflowError("Addition overflow")
    return result


def subtract_in_place(first, second, base=10):
    if compare(first, second) < 0:
        raise ValueError("Negative result")
    borrow = 0
    i = len(first) - 1
    while i >= 0:
        difference = first[i] - second[i] - borrow
        if difference < 0:
            difference += base
            borrow = 1
        else:
            borrow = 0
        first[i] = difference
        i -= 1


def subtract(first, second, base=10):
    validate_number(first, base)
    validate_number(second, base)
    if compare(first, second) >= 0:
        result, smaller = first.copy(), second
    else:
        result, smaller = second.copy(), first
    subtract_in_place(result, smaller, base)
    return result


def multiply(first, second, base=10):
    validate_number(first, base)
    validate_number(second, base)
    # Умножение в столбик; рабочий массив вдвое длиннее результата.
    work = [0] * (MAX_DIGITS * 2)
    i = MAX_DIGITS - 1
    while i >= 0:
        j = MAX_DIGITS - 1
        while j >= 0:
            work[i + j + 1] += first[i] * second[j]
            j -= 1
        i -= 1
    i = len(work) - 1
    while i > 0:
        work[i - 1] += work[i] // base
        work[i] %= base
        i -= 1
    i = 0
    while i < MAX_DIGITS:
        if work[i] != 0:
            raise OverflowError("Multiplication overflow")
        i += 1
    result = zero()
    i = 0
    while i < MAX_DIGITS:
        result[i] = work[MAX_DIGITS + i]
        i += 1
    return result


def divide_with_remainder(first, second, base=10):
    validate_number(first, base)
    validate_number(second, base)
    if is_zero(second):
        raise ZeroDivisionError("Division by zero")
    quotient = zero()
    remainder = [0] * (MAX_DIGITS + 1)
    divisor = [0] * (MAX_DIGITS + 1)
    i = 0
    while i < MAX_DIGITS:
        divisor[i + 1] = second[i]
        i += 1
    i = 0
    while i < MAX_DIGITS:
        j = 0
        while j < MAX_DIGITS:
            remainder[j] = remainder[j + 1]
            j += 1
        remainder[MAX_DIGITS] = first[i]
        digit = 0
        while compare(remainder, divisor) >= 0:
            subtract_in_place(remainder, divisor, base)
            digit += 1
        quotient[i] = digit
        i += 1
    result_remainder = zero()
    i = 0
    while i < MAX_DIGITS:
        result_remainder[i] = remainder[i + 1]
        i += 1
    return quotient, result_remainder


def divide(first, second, base):
    return divide_with_remainder(first, second, base)


def convert(number, from_base, to_base):
    validate_number(number, from_base)
    check_base(to_base)
    result = zero()
    i = 0
    while i < MAX_DIGITS:
        # Схема Горнера над массивом цифр целевой системы.
        carry = number[i]
        j = MAX_DIGITS - 1
        while j >= 0:
            value = result[j] * from_base + carry
            result[j] = value % to_base
            carry = value // to_base
            j -= 1
        if carry != 0:
            raise OverflowError("Base conversion overflow")
        i += 1
    return result


def main():
    c1 = create_number("560111202232646")
    c2 = convert(c1, 10, 2)
    c3 = convert(c2, 2, 5)
    c4 = add(c3, c3, 5)
    c5 = subtract(c4, c3, 5)
    c6 = convert(c5, 5, 7)
    c7 = multiply(c6, c6, 7)
    c8, remainder = divide(c7, c6, 7)
    c9 = convert(c8, 7, 10)
    numbers = (c1, c2, c3, c4, c5, c6, c7, c8, c9)
    bases = (10, 2, 5, 5, 5, 7, 7, 7, 10)
    i = 0
    while i < 9:
        print(f"C{i + 1} (base {bases[i]}): {number_text(numbers[i])}")
        i += 1
    if not is_zero(remainder) or compare(c1, c9) != 0:
        raise RuntimeError("Round-trip conversion failed")
    print("ALL CHECKS PASSED")
    a, b = create_number("1010"), create_number("11")
    print(f"1010 + 11 (base 2) = {number_text(add(a, b, 2))}")
    print(f"1010 - 11 (base 2) = {number_text(subtract(a, b, 2))}")
    print(f"1010 * 11 (base 2) = {number_text(multiply(a, b, 2))}")
    quotient, remainder = divide(a, b, 2)
    print(f"1010 / 11 (base 2) = {number_text(quotient)}; remainder = {number_text(remainder)}")
    print(f"1010 (base 2) -> base 10 = {number_text(convert(a, 2, 10))}")

if __name__ == "__main__":
    main()
