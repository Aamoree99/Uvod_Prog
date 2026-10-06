/*
Cvičení 07 — Úloha 7.5
Název: Dlouhá celá čísla v obecné číselné soustavě
Body: 1/2 bodu; 1/2 bodu; 1/2 bodu; 1/2 bodu; 3 body (v pořadí podúloh)

Zadání:
Funkce z příkladu 7.1 upravte tak, aby pracovaly s čísly v zadané číselné soustavě:

a) Funkce Soucet sečte dvě čísla. [1/2 bodu]

b) Funkce Rozdil odečte menší číslo od většího. [1/2 bodu]

c) Funkce Soucin vynásobí dvě čísla. [1/2 bodu]

d) Funkce Podil vydělí číslo číslem. [1/2 bodu]

e) Napište funkci Prevod, která převede číslo používající způsob uložení cifer big endian z jedné
číselné soustavy do druhé číselné soustavy. Při praktických pokusech používejte číselné soustavy o
maximálním základu 10, snáze se v nich zobrazují výsledky. [3 body]


Ukázka:

int[] c1 = Vytvor("560111202232646");

int[] c2 = Prevod(c1, 10, 2);

// 1111111010110101100001101001111100110100101000110

int[] c3 = Prevod(c2, 2, 5); // 1041403330220232421041

int[] c4 = Soucet(c3, c3, 5); // 2133312210441020342132

int[] c5 = Rozdil(c4, c3, 5); // 1041403330220232421041

int[] c6 = Prevod(c5, 5, 7); // 225656461511451251

int[] c7 = Soucin(c6, c6, 7);

// 55402656051563425266136012062265231

int[] c8 = Podil(c7, c6, 7); // 225656461511451251

int[] c9 = Prevod(c8, 7, 10); // 560111202232646

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
