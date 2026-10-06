/*
Cvičení 11 — Úloha 11.1
Název: Hanojské věže
Body: 5 bodů

Zadání:
V minulé hodině byla představena hra Hanojské věže a její řešení pomocí rekurzivní funkce.
Výsledkem funkce byla posloupnost tahů vedoucích k řešení hry. Správnost výsledku jsme si museli
zkontrolovat externě. Napište funkci HonojTah, která zkontroluje, zda zadaný tah je proveditelný –
splňuje následující dvě podmínky:

a) Na věži, ze které přesouváme disk se nějaký disk nachází

b) Po provedení přesunu nebude na cílové věži větší disk na menším.

Funkce HanojTah rovněž vypíše do zadaného souboru informace o provedeném tahu, včetně informací o
provedených kontrolách. Funkci HanojTah použijte v rekurzivní funkci HanojRek, která řeší hru
Hanojské věže.  [5
bodů]


Ukázka:

Hanoj("Hanoj.txt", 6); // presun z Vez 1 na Vez 2 pres Vez 3


Pokusime se prenest disk z Vez 1 na Vez 3.

Vez 1: 6 5 4 3 2 1

Vez 3: nic

Na zdrojove vezi se nachazi disk 1, lze pokracovat v tahu.

Cilova vez je prazdna, lze provest tah.

Presouvame disk 1 z Vez 1 na vez Vez 3.


. . .


Pokusime se prenest disk z Vez 1 na Vez 2.

Vez 1: 6 5 2

Vez 2: 4 3

Na zdrojove vezi se nachazi disk 2, lze pokracovat v tahu.

Na cilove vezi se nachazi disk 3

Lze polozit mensi disk 2 na vetsi 3.

Presouvame disk 2 z Vez 1 na vez Vez 2.


. . .


Pokusime se prenest disk z Vez 3 na Vez 2.

Vez 3: 1

Vez 2: 6 5 4 3 2

Na zdrojove vezi se nachazi disk 1, lze pokracovat v tahu.

Na cilove vezi se nachazi disk 2

Lze polozit mensi disk 1 na vetsi 2.

Presouvame disk 1 z Vez 3 na vez Vez 2.

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
