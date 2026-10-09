"""
Cvičení 09 — Úloha 9.2
Název: Gaussova eliminační metoda
Body: 6 bodů; 3 body; 1 bod (v pořadí podúloh)

Zadání:
Napište následující funkce pracující s maticí celých čísel long:

a) Napište funkci OdstupnujMatici, která pomocí elementárních řádkových úprav převede zadanou
matici na odstupňovaný tvar. [6 bodů]

b) Napište funkci GaussovaEliminace, která za pomocí funkce OdstupnujMatici z matice reprezentující
nehomogenní soustavu lineárních rovnic vypočte hodnoty jednotlivých neznámých a vrátí je poli
reálných čísel. V případě, že soustava nemá řešení, je vráceno pole nulové délky. V případě, že
soustava má nekonečně mnoho řešení, nastavte volitelné proměnné na hodnoty 0. [3 body]

c) Napište funkci Zkouska. Funkce nejprve dosadí hodnoty ze zadaného pole reprezentujícího hodnoty
neznámých do zadané matice reprezentující nehomogenní soustavu lineárních rovnic. Funkce ověří, zda
pro každou rovnici platí rovnost levé a pravé strany. Tj. hodnoty neznámých nacházející se
v zadaném poli jsou řešením nehomogenní soustavy lineárních rovnic reprezentované zadanou maticí.
[1 bod]


Ukázka:

long[,] M = { { 0, 0, 3, 2 }, { 0, 2, 4, 3 }, { 0, 4, 8, 6 }};

OdstupnujMatici(M);

// { { 0, 2, 4, 3 }, { 0, 0, 3, 2 }, { 0, 0, 0, 0 } };

double[] y = GaussovaEliminace(M); // 0.166666, 0.666666

bool b = Zkouska(M, y); // true

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
    print("This exercise is not implemented yet.")

if __name__ == "__main__":
    main()
