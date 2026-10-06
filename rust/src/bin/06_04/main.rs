/*
Cvičení 06 — Úloha 6.4
Název: Ruleta
Body: 10 bodů

Zadání:
Napište program, který umožní uživateli si zahrát evropskou ruletu. Hra probíhá v sázkových kolech.
Uživatel může v každém sázkovém kole uzavírat libovolný počet (a vzájemné kombinace) sázek dle
tabulky sázek a výher. Uživatel v rámci sázkového kola zadá všechny své sázky formou jednoho
textového řetězce. Formát zadávání sázek navrhněte sami, můžete se inspirovat formátem navrženým
v ukázce. Neplatné části řetězce ignorujte. Následně je náhodně vybráno jedno z čísel 0 až 36.
Uživateli je oznámeno, jak dopadly jeho jednotlivé sázky, jsou mu proplaceny případné výhry.
Uživatel může hru v jakémkoliv kole ukončit, případné sázky jsou anulovány. [10 bodů]



Tabulka sázek a výher

a) jednotlivé číslo (0 až 36), výhra 35x vsazená suma

b) liché číslo (1,3,5…35) nebo sudé číslo (2,4,6, …., 36), výhra 1x vsazená suma

c) malé číslo (1 až 18) nebo vysoké číslo (19 až 36), výhra 1x vsazená suma

d) tucet: první (1 až 12), druhý (13 až 24), třetí (25 až 36), výhra 2x vsazená suma


Ukázka:

Vitejte ve hre ruleta:

Sazka: cislo 0 az 36, licha, suda, nizka, vysoka, 1. az 3. tucet. Za sazku vlozte vsazenou sumu,
sazky lze kombinovat.

Sazka? 5 20 1 25 9 30 Suda 100 vysoka 200 hotovo

Na rulete padlo cislo 24.

Sazka na sude cislo ve vysi 100 byla vyhrana. Konto +100.

Sazka na vysoke cislo ve vysi 200 byla vyhrana. Konto +200.

Sazka na cislo 1 ve vysi 25 byla prohrana. Konto -25.

Sazka na cislo 5 ve vysi 20 byla prohrana. Konto -20.

Sazka na cislo 9 ve vysi 30 byla prohrana. Konto -30.

Stav konta je 222.

Sazka? Super jedeme dal 2 tucet 100, 15 50, suda 200

Na rulete padlo cislo 8.

Sazka na sude cislo ve vysi 200 byla vyhran. Konto +200.

Sazka na 2. tucet ve vysi 100 byla prohrana. Konto -100.

Sazka na cislo 15 ve vysi 50 byla prohrana. Konto -50.

Stav konta je 275.

Sazka? Konec, uz me to staci

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
