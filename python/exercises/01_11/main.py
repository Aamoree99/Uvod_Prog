"""
Cvičení 01 — Úloha 1.11
Název: Rychlost na dálnici
Body: 3 body

Zadání:
Nejnižší povolená rychlost na dálnici je 60 km/h, za její nedodržení je udělena pokuta 1000 Kč.
Nejvyšší povolená rychlost na dálnici je 130 km/h, za její překročení je udělena pokuta 2000 Kč.
V případě výrazného překročení nejvyšší povolené se pokuta zvyšuje o 500 Kč za každých započatých 5
km/h nad 150 km/h.

Ve funkci Main deklarujte jednu celočíselnou proměnnou (pojmenujte je libovolně) a tuto proměnnou
inicializujte hodnotou, které by mohla reprezentovat rychlost automobilu.  Napište program, který
určí výši pokuty odpovídající dané rychlosti. Cíl příkladu a jeho výsledek vypište vhodným způsobem
na obrazovku. [3 body]


Ukázka:

Hodnota proměnné je 146.

Rychlost 146 km/h je pokutována částkou 2000 Kč.

Hodnota proměnné je 162.

Rychlost 162 km/h je pokutována částkou 3500 Kč.

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
    speed = 146
    if speed < 60:
        fine = 1000
    elif speed <= 130:
        fine = 0
    elif speed <= 150:
        fine = 2000
    else:
        fine = 2000 + ((speed - 150 + 4) // 5) * 500
    print(f"A speed of {speed} km/h incurs a fine of {fine} CZK.")

if __name__ == "__main__":
    main()
