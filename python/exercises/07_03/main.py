"""
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


# Дробь представлена тройкой: числитель, знаменатель, знак.
def gcd_big(first, second):
    a, b = first.copy(), second.copy()
    while not is_zero(b):
        _, remainder = divide_with_remainder(a, b)
        a, b = b, remainder
    return a


def divide_exact(first, second):
    quotient, remainder = divide_with_remainder(first, second)
    if not is_zero(remainder):
        raise ValueError("Division is not exact")
    return quotient


def create_fraction(text):
    slash = 0
    while slash < len(text) and text[slash] != '/':
        slash += 1
    if slash == 0 or slash + 1 >= len(text):
        raise ValueError("Invalid fraction")
    negative = text[0] == '-'
    start = 1 if text[0] in ('-', '+') else 0
    numerator = create_number(text[start:slash])
    denominator = create_number(text[slash + 1:])
    if is_zero(denominator):
        raise ZeroDivisionError("Denominator cannot be zero")
    return numerator, denominator, negative and not is_zero(numerator)


def fraction_text(fraction):
    numerator, denominator, negative = fraction
    return ('-' if negative else '') + number_text(numerator) + '/' + number_text(denominator)


def reduce_fraction(fraction):
    numerator, denominator, negative = fraction
    if is_zero(numerator):
        return zero(), create_number('1'), False
    gcd = gcd_big(numerator, denominator)
    return divide_exact(numerator, gcd), divide_exact(denominator, gcd), negative


def add_fractions(first, second):
    a = multiply(first[0], second[1])
    b = multiply(second[0], first[1])
    denominator = multiply(first[1], second[1])
    if first[2] == second[2]:
        numerator, negative = add(a, b), first[2]
    elif compare(a, b) >= 0:
        numerator, negative = subtract(a, b), first[2]
    else:
        numerator, negative = subtract(b, a), second[2]
    return reduce_fraction((numerator, denominator, negative))


def multiply_fractions(first, second):
    return reduce_fraction((multiply(first[0], second[0]), multiply(first[1], second[1]), first[2] != second[2]))


def divide_fractions(first, second):
    if is_zero(second[0]):
        raise ZeroDivisionError("Division by zero fraction")
    return reduce_fraction((multiply(first[0], second[1]), multiply(first[1], second[0]), first[2] != second[2]))


def main():
    first = create_fraction('-56890/1223366')
    second = create_fraction('2565689841/2365')
    print(f"Z1 = {fraction_text(first)}\nZ2 = {fraction_text(second)}")
    first, second = reduce_fraction(first), reduce_fraction(second)
    print(f"Reduced Z1 = {fraction_text(first)}\nReduced Z2 = {fraction_text(second)}")
    print(f"Z1 + Z2 = {fraction_text(add_fractions(first, second))}")
    print(f"Z1 * Z2 = {fraction_text(multiply_fractions(first, second))}")
    print(f"Z2 / Z1 = {fraction_text(divide_fractions(second, first))}")
    print(f"12/18 reduced = {fraction_text(reduce_fraction(create_fraction('12/18')))}")
    print(f"-1/3 + 1/3 = {fraction_text(add_fractions(create_fraction('-1/3'), create_fraction('1/3')))}")
    a, b = create_fraction('2/3'), create_fraction('3/4')
    print(f"2/3 * 3/4 = {fraction_text(multiply_fractions(a, b))}")
    print(f"2/3 / 3/4 = {fraction_text(divide_fractions(a, b))}")

if __name__ == "__main__":
    main()
