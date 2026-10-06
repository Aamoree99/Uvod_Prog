/*
Cvičení 07 — Úloha 7.1
Název: Dlouhá celá čísla
Body: 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 2 body; 3 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s dlouhými celými čísly, které používají způsob uložení cifer v poli big
endian (na přednášce byl použit způsob little endian).

a) Funkci Vytvor, která převede textový řetězec na číslo. [1 bod]

b) Funkci Vypis, která vypíše číslo na obrazovku bez zbytečných nul. [1 bod]

c) Funkce Compare porovná dvě čísla. Je-li první číslo menší než druhé číslo, je vrácena hodnota
-1. Je-li první číslo větší než druhé číslo, je vrácena hodnota 1. Jsou-li si obě čísla rovna, je
vrácena hodnota 0. [1 bod]

d) Funkce Soucet sečte dvě čísla. [1 bod]

e) Funkce Rozdil odečte menší číslo od většího. [1 bod]

f) Funkce Soucin vynásobí dvě čísla. [2 body]

g) Funkce Podil vydělí číslo číslem. [3 body]


Ukázka:

int[] c1 = Vytvor("560111202232646");

int[] c2 = Vytvor("560100600690986");

Vypis(c1); // 560111202232646

int i = Compare(c1, c2); // 1

int[] c3 = Soucet(c1, c2);  // 1120211802923632

int[] c4 = Rozdil(c1, c2);  // 10601541660

int[] c5 = Soucin(c1, c4); // 5938042244702081581032360

int[] c6 = Podil(c1, c4); // 52832

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
*/

// Решение пока не написано. Этот entrypoint нужен только для сборки Cargo.
fn main() {
    println!("Reseni teto ulohy zatim neni implementovano.");
}
