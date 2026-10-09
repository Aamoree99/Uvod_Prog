"""
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
"""

MAX_DIGITS = 100

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


def divide(first, second):
    quotient, _ = divide_with_remainder(first, second)
    return quotient


def main():
    first = create_number("560111202232646")
    second = create_number("560100600690986")
    print(f"First number: {number_text(first)}\nSecond number: {number_text(second)}")
    print(f"Comparison: {compare(first, second)}")
    print(f"Sum: {number_text(add(first, second))}")
    difference = subtract(first, second)
    print(f"Difference: {number_text(difference)}")
    print(f"Product: {number_text(multiply(first, difference))}")
    print(f"Quotient: {number_text(divide(first, difference))}")
    a, b = create_number("25"), create_number("100")
    print(f"Zero: {number_text(create_number('0000'))}")
    print(f"25 + 100 = {number_text(add(a, b))}")
    print(f"100 - 25 = {number_text(subtract(b, a))}")
    print(f"25 * 100 = {number_text(multiply(a, b))}")
    print(f"100 / 25 = {number_text(divide(b, a))}")

if __name__ == "__main__":
    main()
