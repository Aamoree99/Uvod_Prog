/*
Cvičení 06 — Úloha 6.1
Název: Práce s textovými řetězci
Body: 2 body; 2 body; 2 body; 1 bod; 1 bod; 4 body (v pořadí podúloh)

Zadání:
Slovo je řetězec složený výhradně z písmen, který je maximální délky, nejde tedy prodloužit o další
písmeno. Dvě slova jsou od sebe oddělena jedním nebo více znaky, které nejsou písmeny. Pokud se
před slovem či za slovem nachází znak, tak tento znak není písmenem. Napište následující funkce pro
práci s textovými řetězci bez použití funkcí Split a Distinct. Původní řetězec nesmíte modifikovat,
ani od něj vytvářet kopie:

a) Funkce vracející počet různých písmen anglické abecedy, které zadaný řetězec obsahuje. Velikost
písmen se nerozlišuje. [2 body]

b) Funkce vracející počet slov v zadaném řetězci. [2 body]

c) Funkce vracející slovo první výskyt slova na zadané pozice v zadaném řetězci. Pokud se na zadané
pozici nachází písmeno, je vráceno slovo, jehož součástí toto písmeno je. V tomto případě bude
nalezené slovo často začínat před zadanou pozicí. Pokud se na zadané pozici nenachází písmeno, je
vráceno první slovo nacházející se napravo od zadané pozice v zadaném řetězci. Pokud není nalezeno
žádné slovo, je vrácen prázdný řetězec. [2 body]

d) Funkce vracející nejdelší slovo v zadaném řetězci. Pokud jsou dvě slova stejně dlouhá, je
vráceno slovo, které se v řetězci vyskytuje jako první. Pokud není nalezeno žádné slovo, je vrácen
prázdný řetězec. [1 bod]

e) Funkce vracející lexikograficky (abecedně) největší slovo v zadaném řetězci. Pokud není nalezeno
žádné slovo, je vrácen prázdný řetězec. [1 bod]

f) Funkce vracející nejčastěji se vyskytující slovo v zadaném řetězci. Pokud se vyskytují dvě slova
stejně často, je vráceno slovo, které se v řetězci vyskytuje jako první. Pokud není nalezeno žádné
slovo, je vrácen prázdný řetězec. Dejte pozor na situace, kdy jedno slovo je podslovem jiného
slova. V řetězci "Ja a Jana" má slovo "a" pouze jeden výskyt. [4 body]


Ukázka:

string s1 = "Alena ma 2 ruce.";

int ruznych = RuznychPismen(s1); // 8

int pocet = PocetSlov(s1); // 3

string s2 = PrvniSlovo(s1, 2); // "Alena"

string s3 = PrvniSlovo(s1, s1.Length - 1); // ""

string s4 = NejdelsiSlovo(s1); // "Alena"

string s5 = NejvetsiSlovo(s1); // "ruce"

string s6 = "Mama ma 2 ruce a v ruce hul";

string s7 = NejcastejsiSlovo(s1 + s6); // "ruce"

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
