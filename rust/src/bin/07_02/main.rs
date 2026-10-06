/*
Cvičení 07 — Úloha 7.2
Název: Dlouhá reálná čísla
Body: 4 body; 1 bod; 1 bod; 5 bodů; 4 body; 3 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s dlouhými reálnými čísly v plovoucí řádové čárce, které používají způsob
uložení cifer v poli little endian (tento způsob byl použit na přednášce).

a) Funkci Vytvor, která převede textový řetězec na číslo. Textový řetězec může (ale nemusí)
obsahovat znak pro exponent. Př.: 7896e4 = 78960000; 5636.7e-2 =56.367; [4 body]

b) Funkci Vypis, která vypíše číslo na obrazovku bez zbytečných nul. [1 bod]

c) Funkce Compare porovná dvě čísla. Je-li první číslo menší než druhé číslo, je vrácena hodnota
-1. Je-li první číslo větší než druhé číslo, je vrácena hodnota 1. Jsou-li si obě čísla rovna, je
vrácena hodnota 0. [1 bod]

d) Funkce Soucet sečte dvě čísla. V závislosti na znaménku sčítaných čísel bude docházet ke sčítání
(př. 2 + 3) nebo odečítání (př. 2 + -3) absolutních hodnot čísel. Na rozdíl od implementace celých
čísel vyřešte i možná přetečení a podtečení výsledku. [5 bodů]

e) Funkce Soucin vynásobí dvě čísla. [4 body]

f) Funkce Podil vydělí číslo číslem. [3 body]


Ukázka:

Real r1 = Vytvor("-256.2e-1"); // - 2.562e1

Real r2 = Vytvor("-9534.07256e15"); // -9.53407256e18

Real r3 = Vytvor("9534.07256e15"); // 9.53407256e18

Vypis(r1); Vypis(r2); Vypis(r3);

int i = Compare(r1,r2); // -1

Real r4 = Soucet(r2, r2);  // -1.906814512e19

Real r5 = Soucet(r1, r3);  // 9.53407255999999997438e18

Real r6 = Soucet(r2, r3);  // 0

Real r7 = Soucin(r1, r2);  // 2.442629389872e20

Real r8 = Podil(r2, r1);  // 3.7213397970335675253...e17

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
