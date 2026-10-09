"""
Cvičení 01 — Úloha 1.5
Název: Prostřední ze tří čísel
Body: 6 bodů

Zadání:
Ve funkci Main deklarujte tři celočíselné proměnné (pojmenujte je libovolně) a tyto proměnné
inicializujte hodnotami (ideálně různými). Napište program, který určí prostřední (= druhou
nejvyšší) z hodnot výše uvedených tří proměnných. Cíl příkladu a jeho výsledek vypište vhodným
způsobem na obrazovku. [6 bodů]


Ukázka:

Hodnota první proměnné je 11, hodnota druhé proměnné je 18, hodnota třetí proměnné je 5.

Prostřední z hodnot 11, 18 a 5 je 11.

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
    a, b, c = 11, 18, 5
    if (b <= a <= c) or (c <= a <= b):
        middle = a
    elif (a <= b <= c) or (c <= b <= a):
        middle = b
    else:
        middle = c
    print(f"The middle of {a}, {b} and {c} is {middle}.")

if __name__ == "__main__":
    main()
