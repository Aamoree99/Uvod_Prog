"""
Cvičení 02 — Úloha 2.7
Název: Dokonalá čísla
Body: 3 body

Zadání:
Vypište všechna dokonalá čísla menší než 10 000. Číslo je dokonalé, pokud je rovno součtu svých
dělitelů, kromě sebe sama. Např. 6 = 1+2+3; 28 = 1+2+4+7+14 [3 body]


Ukázka:

Vsechna dokonala cisla mensi nez 10000: 6, 28, . . . 8128

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
    print("All perfect numbers less than 10000:")
    number = 2
    while number < 10000:
        divisor = 1
        total = 0
        while divisor < number:
            if number % divisor == 0:
                total += divisor
            divisor += 1
        if total == number:
            print(number, end=" ")
        number += 1
    print()

if __name__ == "__main__":
    main()
