/*
Cvičení 03 — Úloha 3.8
Název: Den v týdnu
Body: 4 body

Zadání:
Napište funkci, která k zadanému datu (pomocí struktury) určí den v týdnu (výčtový typ). Funkce
zohledňuje přestupné roky.  [4 body]

Správnost programu si můžete ověřit na online datumové kalkulačce

http://www.timeanddate.fasterreader.eu/pages/cs/day-of-week-cs.html

Ukázka:

d.den = 15; d.mesic = 3; d.rok = 1966;

den = DenVTydnu(d); // Utery

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

#include <iostream>
#include <iomanip>
#include <string>

// Дата хранит день, месяц и год.
struct Datum { int den; int mesic; int rok; };
// Год високосный при делимости на 400 либо на 4, но не на 100.
static bool je_prestupny_rok(int rok) {
    if (rok % 400 == 0) return true;
    if (rok % 100 == 0) return false;
    if (rok % 4 == 0) return true;
    return false;
}
// Февраль зависит от года; остальные месяцы имеют фиксированную длину.
static int pocet_dnu_v_mesici(int mesic, int rok) {
    if (mesic == 1 || mesic == 3 || mesic == 5 || mesic == 7 || mesic == 8 || mesic == 10 || mesic == 12) return 31;
    if (mesic == 4 || mesic == 6 || mesic == 9 || mesic == 11) return 30;
    return je_prestupny_rok(rok) ? 29 : 28;
}
enum class DenVTydnu { Pondeli, Utery, Streda, Ctvrtek, Patek, Sobota, Nedele };
// Как в Rust-решении, дата предполагается корректной и не раньше 1900 года.
static DenVTydnu den_v_tydnu(Datum datum) {
    int pocet_dnu=0, rok=1900;
    while (rok < datum.rok) { pocet_dnu += je_prestupny_rok(rok) ? 366 : 365; rok += 1; }
    int mesic=1;
    while (mesic < datum.mesic) { pocet_dnu += pocet_dnu_v_mesici(mesic,datum.rok); mesic += 1; }
    // До первого числа месяца прошло ноль полных дней этого месяца.
    pocet_dnu += datum.den - 1;
    // 1 января 1900 года — понедельник; остаток даёт смещение по неделе.
    int den=pocet_dnu % 7;
    if (den==0) return DenVTydnu::Pondeli;
    if (den==1) return DenVTydnu::Utery;
    if (den==2) return DenVTydnu::Streda;
    if (den==3) return DenVTydnu::Ctvrtek;
    if (den==4) return DenVTydnu::Patek;
    if (den==5) return DenVTydnu::Sobota;
    return DenVTydnu::Nedele;
}
static std::string den_na_text(DenVTydnu den) {
    if (den==DenVTydnu::Pondeli) return "Pondeli";
    if (den==DenVTydnu::Utery) return "Utery";
    if (den==DenVTydnu::Streda) return "Streda";
    if (den==DenVTydnu::Ctvrtek) return "Ctvrtek";
    if (den==DenVTydnu::Patek) return "Patek";
    if (den==DenVTydnu::Sobota) return "Sobota";
    return "Nedele";
}
int main() {
    std::cout << std::boolalpha;
    Datum d=Datum{6,10,2026};
    DenVTydnu den=den_v_tydnu(d);
    std::cout << "Datum:      " << std::setfill('0') << std::setw(2) << d.den << std::setfill(' ') << "." << std::setfill('0') << std::setw(2) << d.mesic << std::setfill(' ') << "." << d.rok << '\n';
    std::cout << "Den tydne:  " << den_na_text(den) << '\n';
}
