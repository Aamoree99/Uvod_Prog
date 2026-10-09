"""
Cvičení 04 — Úloha 4.3
Název: Zaokrouhlení pole
Body: 2 body

Zadání:
Napište funkci, která pro zadané pole reálných čísel vrátí pole celých čísel. Hodnoty prvků v novém
poli vzniknou zaokrouhlením hodnot prvků z původního pole. [2 body]


Ukázka:

double[] pole = { 5.12, -3.5996, 156, 7.9 };

zaokrouhli = ZaokrouhliPole(pole); // 5, -4, 156, 8

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

def round_array(array):
    result = [0] * len(array)
    i = 0
    while i < len(array):
        # Как Rust round: половины округляем от нуля, а не к чётному.
        value = array[i]
        result[i] = int(value + 0.5) if value >= 0 else int(value - 0.5)
        i += 1
    return result


def main():
    array = [5.12, -3.5996, 156.0, 7.9]
    print(f"Original: {array}")
    print(f"Rounded: {round_array(array)}")

if __name__ == "__main__":
    main()
