"""
Cvičení 07 — Úloha 7.4
Název: Římská čísla
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Římská čísla jsou příkladem nepoziční číselné soustavy. Hodnoty číslic nezávisí na jejich pozici
v zápise čísla. Hodnoty číslic jsou následující I = 1, V = 5, X = 10, L = 50, C = 100, D = 500, M =
1000. Existují i dvojice číslic, které se považují za jednu číslici: IV  = 4, IX = 9, XL = 40, XC =
90, CD 400, CM 900. Číslice jsou v zápisu čísla uspořádány od nejvyšší hodnoty po nejnižší.
Například číslo 2016 se zapíše jako MMXVI. Například číslo 1949 se zapíše zkráceným zápisem jako
MCMXLIX. Například výraz IM, který měl mít hodnotu 999, odporuje výše uvedeným pravidlům a není
povolen. Napište následující funkce:

a) Funkce DecRoman převede číslo v desítkové soustavě na římské číslo. [1 bod]

b) Funkce RomanDec převede římské číslo na číslo v desítkové soustavě [1 bod].


Ukázka:

string s = DecRoman(1969); // "MCMLXIX"

int i = RomanDec("MMCDXXXVII"); // 2437


Správnost programu si můžete ověřit na online konvertoru:

http://www.convertit.com/Go/ConvertIt/Calculators/Math/Roman_Numerals_Converter.ASP

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

def decimal_to_roman(number):
    if not 1 <= number <= 3999:
        raise ValueError("Number must be between 1 and 3999")
    values = (1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1)
    symbols = ("M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I")
    result = ""
    i = 0
    while i < len(values):
        while number >= values[i]:
            result += symbols[i]
            number -= values[i]
        i += 1
    return result


def roman_value(character):
    symbols = "IVXLCDM"
    values = (1, 5, 10, 50, 100, 500, 1000)
    i = 0
    while i < len(symbols):
        if character == symbols[i]:
            return values[i]
        i += 1
    return 0


def roman_to_decimal(text):
    if text == "":
        return -1
    result = i = 0
    while i < len(text):
        current = roman_value(text[i])
        if current == 0:
            return -1
        if i + 1 < len(text) and current < roman_value(text[i + 1]):
            result -= current
        else:
            result += current
        i += 1
    if not 1 <= result <= 3999 or decimal_to_roman(result) != text:
        return -1
    return result


def main():
    print(f"1969 -> {decimal_to_roman(1969)}")
    print(f"MMCDXXXVII -> {roman_to_decimal('MMCDXXXVII')}")
    examples = (4, 9, 49, 999, 2016, 3999)
    i = 0
    while i < len(examples):
        print(f"{examples[i]} -> {decimal_to_roman(examples[i])}")
        i += 1
    invalid = ('IM', 'IIII', 'VX', 'IC', 'ABC', '')
    i = 0
    while i < len(invalid):
        print(f"{invalid[i]!r} -> {roman_to_decimal(invalid[i])}")
        i += 1
    number = 1
    while number <= 3999:
        if roman_to_decimal(decimal_to_roman(number)) != number:
            raise RuntimeError("Round-trip test failed")
        number += 1
    print("All numbers 1-3999 passed: True")

if __name__ == "__main__":
    main()
