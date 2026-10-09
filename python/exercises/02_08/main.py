"""
Cvičení 02 — Úloha 2.8
Název: Armstrongova čísla
Body: 4 body

Zadání:
Vypište všechna Armstrongova čísla menší než 100 000. Armstrongovo číslo řádu n je n-ciferné číslo,
které je rovno součtu n-tých mocnin svých cifer. Například jakékoliv jednociferné číslo je
Armstrongovo číslo řádu 1, číslo 153 = 1*1*1+5*5*5+3*3*3 je Armstrongovo číslo řádu 3. [4 body]


Ukázka:

Vsechna Armstrongova cisla mensi nez 100000: 1, 2, . . . 93084

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

def main():
    number = 1
    while number < 100000:
        remaining = number
        digits = 0
        while remaining > 0:
            digits += 1
            remaining //= 10
        remaining = number
        total = 0
        while remaining > 0:
            digit = remaining % 10
            power = 1
            count = 0
            while count < digits:
                power *= digit
                count += 1
            total += power
            remaining //= 10
        if total == number:
            print(number, end=" ")
        number += 1
    print()

if __name__ == "__main__":
    main()
