"""
Cvičení 10 — Úloha 10.1
Název: Rekurze s jedním voláním
Body: 2 body; 2 body; 3 body; 3 body; 3 body (v pořadí podúloh)

Zadání:
S využitím rekurze napište následující funkce. Na začátku každé funkce bude výpis (v režimu Append)
do souboru log.txt. Zapisovat budeme: (1) volání funkce a hodnotu parametrů; (2) ukončení funkce a
návratová hodnota. Otevřený soubor bude jedním z parametrů funkce.

a) Rekurzivní funkce zanořující se na podproblém velikosti n - 1, která vrátí hodnotu nejmenšího
prvku v poli. [2 body]

b) Rekurzivní funkce zanořující se na podproblém velikosti n - 1, která vrátí součet prvků v poli,
které mají kladnou hodnotu. [2 body]

c) Rekurzivní funkce zanořující se na dva podproblémy velikosti n / 2, která vrátí hodnotu
nejmenšího prvku v poli. [3 body]

d) Rekurzivní zanořující se na dva podproblémy velikosti n / 2, která vrátí součet prvků v poli,
které mají kladnou hodnotu. [3 body]

e) Rekurzivní funkce, která spočte největšího společného dělitele dvou čísel Euklidovým algoritmem.
[3 body]


Ukázka:

int[] p = {7, 5, 6, -3, -2, 8, -5};

int i1 = Min1(p); // -5

int i2 = Sum1(p); // 26

int i3 = Min2(p); // -5

int i4 = Sum2(p); // 26

int i5 = Euklid(21, 77); // 7



Start Min1(p)

Start MinRek1(p,0,6)

Start MinRek1(p,0,5)

Start MinRek1(p,0,4)

Start MinRek1(p,0,3)

Start MinRek1(p,0,2)

Start MinRek1(p,0,1)

Start MinRek1(p,0,0)

Konec MinRek1(p,0,0) = 7

Konec MinRek1(p,0,1) = 5

Konec MinRek1(p,0,2) = 5

Konec MinRek1(p,0,3) = -3

Konec MinRek1(p,0,4) = -3

Konec MinRek1(p,0,5) = -3

Konec MinRek1(p,0,6) = -5

Konec Min1(p) = -5



                         Start Sum2(p)

                         Start SumRek2(p,0,6)

                         Start SumRek2(p,0,2)

                         Start SumRek2(p,0,0)

                         Konec SumRek2(p,0,0) = 7

                         Start SumRek2(p,2,2)

                         Konec SumRek2(p,2,2) = 6

                         Konec SumRek2(p,0,2) = 18

                         Start SumRek2(p,4,6)

                         Start SumRek2(p,4,4)

                         Konec SumRek2(p,4,4) = 0

                         Start SumRek2(p,6,6)

                         Konec SumRek2(p,6,6) = 0

                         Konec SumRek2(p,4,6) = 8

                         Konec SumRek2(p,0,6) = 26

                         Konec Sum2(p) = 26



                                                  Start Euklid(21, 77)

                                                  Start EuklidRek(21, 77)

                                                  Start EuklidRek(77, 21)

                                                  Start EuklidRek(21, 14)

                                                  Start EuklidRek(14, 7)

                                                  Start EuklidRek(7, 0)

                                                  Konec EuklidRek(7, 0) = 7

                                                  Konec EuklidRek(14, 7) = 7

                                                  Konec EuklidRek(21, 14) = 7

                                                  Konec EuklidRek(77, 21) = 7

                                                  Konec EuklidRek(21, 77) = 7

                                                  Konec Euklid(21, 77) = 7

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
