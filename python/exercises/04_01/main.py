"""
Cvičení 04 — Úloha 4.1
Název: Sestupné setřídění
Body: 1 bod

Zadání:
Napište funkci, která své dva parametry setřídí sestupně. [1 bod]


Ukázka:

int a = 5, b = 7;

SetridSestupne(ref a, ref b); // a = 7; b = 5;

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

def sort_descending(a, b):
    if a < b:
        a, b = b, a
    return a, b


def main():
    a, b = 5, 7
    print(f"Before: a = {a}, b = {b}")
    a, b = sort_descending(a, b)
    print(f"After: a = {a}, b = {b}")

if __name__ == "__main__":
    main()
