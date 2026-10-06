/*
Cvičení 06 — Úloha 6.3
Název: Naše vlastní řetězce
Body: 2 body; 2 body; 1 bod; 2 body; 2 body; 4 body (v pořadí podúloh)

Zadání:
Napište funkce pracující s polem znaků char[], které budou mít stejnou funkčnost jako funkce pro
textové řetězce string. (Aniž byste pole znaků char[], na textový řetězec string převedli).

a) Funkce lexikografického porovnání řetězců s1 a s2: Compare(s1, s2). [2 body]

b) Funkce vracející index prvního výskytu hledaného řetězce s2 v řetězci s1. Řetězec s1 se
prohledává od zadané pozice start: IndexOf(s1, s2, start). [2 body]

c) Funkce vracející podřetězec zadaného řetězce s1. Podřetězec začíná na zadané pozici a má zadanou
délku: Substring(s1, start, delka). [1 bod]

d) Funkce vracející řetězec vzniklý vložením řetězce s2 do řetězce s1 na pozici zadanou indexem:
Insert(s1, index, s2). [2 body]

e) Funkce vracející řetězec vzniklý odstraněním podřetězce ze zadaného řetězce s1. Z řetězce s1 se
odstraní podřetězec určený indexem (v rámci řetězce s1) svého prvního znaku a svojí délkou:
Remove(s1, index, delka). [2 body]

f) Funkce vracející řetězec vzniklý z řetězce s1 nahrazením všech výskytů řetězce s2 za řetězec s3:
Replace (s1, s2, s3). [4 body]


Ukázka:

char[] s1 = "klokan".ToCharArray();

char[] s2 = "klobouk".ToCharArray();

int i1 = Compare(s1, s2); // 1

int i2 = IndexOf(s2, "o".ToCharArray(), 3); // 4

char[] s3 = Substring(s2, 2, 3); // 'o''b''o'

char[] s4 = Insert(s1, 1, s3); // 'k''o''b''o''l''o''k''a''n'

char[] s5 = Remove(s2, 3, 2); // 'k''l''o''u''k''

char[] s6 = Replace(s5, "k".ToCharArray(), s3);

// 'o''b''o''l''o''u''o''b''o'

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
