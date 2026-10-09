"""
Cvičení 09 — Úloha 9.1
Název: Matice
Body: 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod; 1 bod (v pořadí podúloh)

Zadání:
Napište následující funkce pracující s maticí celých čísel long:

a) Funkci, která vytvoří matici zadaného rozměru naplněnou prvky s náhodnými hodnotami. Hodnoty
volte z intervalu [-4,+4], je to výhodné pro použití v dalších příkladech. [1 bod]

b) Funkci, která zadanou matici uloží do textového souboru. Na prvním řádku bude uložen rozměr
matice. Na dalších řádcích budou uloženy jednotlivé řádky matice [1 bod]

c) Funkci, která z textového souboru načte matici (zadanou formou uvedenou v 1.b). [1 bod]

d) Funkci, která vytvoří transponovanou matici (= záměna řádků za sloupce). [1 bod]

e) Funkci testující, zda zadaná matice je symetrická. Matice je symetrická, pokud pro každý prvek
nacházející se v řádku r a sloupci s, platí, že jeho hodnota je rovna hodnotě prvku v řádku s a
sloupci r. [1 bod].

f) Funkce vracející pole obsahující diagonálu matice. Diagonála matice je tvořena prvky, které se
nacházejí v řádku se stejným pořadovým číslem, jako je pořadové číslo sloupce, ve kterém se
nacházejí. [1 bod].

g) Funkci testující, zda matice je horní trojúhelníková. Tj. Matice je čtvercová a všechny prvky
nacházející se pod diagonálou mají nulovou hodnotu. [1 bod].

h) Funkci vracející pořadové číslo sloupce, ve kterém se na zadaném řádku nachází první nenulový
prvek. Tj. Na zadaném řádku určí počet prvků s nulovou hodnotou nacházejících se nalevo od prvního
nenulového prvku zleva [1 bod].

i) Funkci testující, zda matice je odstupňovaná. V odstupňované matici je na každém řádku počet nul
zleva alespoň o jednu nulu větší než na předchozím řádku. [1 bod].

j) Funkci testující, zda matice je jednotková. Jednotková matice je čtvercová matice, která má na
diagonále prvky s hodnotou jedna, ostatní prvky mají nulovou hodnotu. [1 bod].


Ukázka:

long[,] M1 = NahodnaMatice(2, 3);

UlozMatici(M1, "matice1.txt");

long[,] M2 = NactiMatici("matice1.txt");

long[,] M3 = { { 2, 3, 1 }, { 7, 4, 6 } };

long[,] M4 = TransponovanaMatice(M3);

// { {2, 7 }, { 3, 4 }, { 1, 6 } }

long[,] M5 = { { 2, 6, 1 }, { 6, 4, 3 }, { 1, 3, 7 } };

bool b1 = MaticeJeSymetricka(M5); // true

long[] v1 = DiagonalaMatice(M5); // 2, 4, 7

long[,] M6 = { { 4, 5, 2 }, { 0, 3, 6 }, { 0, 0, 9 } };

bool b2 = MaticeJeHorniTrojuhelnikova(M6); // true

int i1 = PocetNulZleva(M6, 2); // 2

long[,] M7 = { { 7, 1, 2 }, { 0, 0, 6 } };

bool b3 = MaticeJeOdstupnovana(M7); // true

bool b4 = MaticeJeJednotkova(M6); // false

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
