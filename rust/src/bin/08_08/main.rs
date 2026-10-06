/*
Cvičení 08 — Úloha 8.8
Název: Formátování textu
Body: 2 body; 2 bod; 2 body (v pořadí podúloh)

Zadání:
Napište funkci, která ze zadaného textového souboru vytvoří druhý textový soubor, ve kterém bude
text zalomen na zadanou délku řádky (obdoba funkcí z MS Word). Řádek bude naplněn slovy tak, že
nepůjde na něj žádné další slovo přidat bez porušení požadované délky řádku. Za slovo se (na rozdíl
od předchozích příkladů) považuje jakýkoliv řetězec z jiných než bílých znaků. Slova mohou být
kombinací písmen, čísel, interpunkčních znamének atd. Bílé znaky a řídící znaky jsou odstraněny a
nahrazeny jednou mezerou mezi každými sousedními řetězci. Slovo nesmí být rozděleno na více řádků
s výjimkou případu, kdy je delší než maximální délka řádku. Zbylé místo na řádku je vyplněno
mezerami. Vytvořte výčtový typ Format, který bude určovat, jakým způsobem se umísťují výplňkové
mezery na řádce.

a) Text se bude formátovat směrem doleva. Všechny výplňkové mezery jsou přidány za poslední slovo.
[2 body]

b) Text se bude formátovat směrem doprava. Všechny výplňkové mezery jsou přidány před první slovo.
[2 bod]

c) Text se bude formátovat na střed. Všechny výplňkové mezery jsou rozděleny na dvě poloviny.
V případě lichého počtu mezer bude druhá polovina o mezeru větší. První polovina je přidána před
první slovo, druhá polovina za poslední slovo [2 body]


Ukázka:

VypisFormat("aaa.txt", "doleva.txt", Format.DoLeva, 20);

/* Nadpis textu Tohle

 * je 5. ukazkovy

 * textovy soubor.

 */

VypisFormat("aaa.txt", "doprava.txt", Format.DoPrava, 20);

/*  Nadpis textu Tohle

 *      je 5. ukazkovy

 *     textovy soubor.

 */

VypisFormat("aaa.txt", "nastred.txt", Format.NaStred, 20);

/* Nadpis textu Tohle

 *   je 5. ukazkovy

 *  textovy soubor.

 */

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
