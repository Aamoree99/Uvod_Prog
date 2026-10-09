"""
Cvičení 03 — Úloha 3.4
Název: Komplexní čísla
Body: 1/2 bodu; 1/2 bodu; 1 bod (v pořadí podúloh)

Zadání:
Vytvořte strukturu pro reprezentaci komplexních čísel v algebraickém tvaru. Napište funkce pro
následující operace

a) sčítání komplexních čísel [1/2 bodu]

b) odečítání komplexních čísel [1/2 bodu]

c) násobení komplexních čísel [1 bod]




Ukázka:

x.re = 5.2; x.im = 3.25;

y.re = 1.6; y.im = -4;

a = secti(x, y); // 6.8 - 0.75i

b = odecti(x, y); // 3.6 + 7.25i

c = vynasob(x, y); // 21.32 - 15.6i

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

# Комплексное число представлено парой (вещественная, мнимая части).
def add(x, y):
    return x[0] + y[0], x[1] + y[1]


def subtract(x, y):
    return x[0] - y[0], x[1] - y[1]


def multiply(x, y):
    return x[0] * y[0] - x[1] * y[1], x[0] * y[1] + x[1] * y[0]


def main():
    x, y = (5.2, 3.25), (1.6, -4.0)
    results = (add(x, y), subtract(x, y), multiply(x, y))
    names = ("Sum", "Difference", "Product")
    i = 0
    while i < 3:
        print(f"{names[i]}: {results[i][0]:.4f} {results[i][1]:+.4f}i")
        i += 1

if __name__ == "__main__":
    main()
