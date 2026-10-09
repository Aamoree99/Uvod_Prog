"""
Cvičení 02 — Úloha 2.5
Název: Nejmenší společný násobek
Body: 3 body

Zadání:
Uživatel zadá dvě čísla z klávesnice. Vypište jejich nejmenší společný násobek. Nejmenší společný
násobek čísel x a y je nejmenší číslo n takové, že x dělí n beze zbytku a zároveň y dělí n beze
zbytku.  [3 body]



Ukázka:

Spocteme nejmensi spolecny nasobek cisel

Zadej prvni cislo: 21

Zadej druhe cislo: 30

Nejmensi spolecny nasobek je 210

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

def least_common_multiple(a, b):
    # Нулевые и отрицательные входы не должны приводить к зависанию.
    if a == 0 or b == 0:
        return 0
    if a < 0:
        a = -a
    if b < 0:
        b = -b
    multiple = a
    while multiple % b != 0:
        multiple += a
    return multiple


def main():
    a = int(input("Enter the first number: "))
    b = int(input("Enter the second number: "))
    print(f"The least common multiple is {least_common_multiple(a, b)}")

if __name__ == "__main__":
    main()
