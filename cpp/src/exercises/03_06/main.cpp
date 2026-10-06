/*
Cvičení 03 — Úloha 3.6
Název: Kámen, nůžky papír
Body: 5 bodů

Zadání:
Napište program, který si s uživatelem zahraje hru kámen, nůžky, papír. Hraje se na dvě vítězná
kola. Použijte výčtový typ. V každém kole si  uživatel zvolí jeden z výše uvedených
předmětů předmětů, počítač svůj předmět vylosuje náhodně. Kámen poráží nůžky, nůžky poráží papír,
papír poráží kámen. Pokud si uživatel i počítač zvolili stejný předmět, dané kolo se opakuje do té
doby, dokud nebudou zvoleny různé předměty. [5 bodů]

Ukázka:

Vitejte ve hre kamen – nuzky – papir

Hrajeme na dve vitezna kola

Zadejte svuj tah: 6

Tento tah je neplatny

Zadejte svuj tah: kamen

Hrac zahral Kamen, pocitac zahral Kamen

Toto kolo je nerozhodne

Celkove skore: hrac 0 – pocitac 0

Zadejte svuj tah: papir

Hrac zahral Papir, pocitac zahral Nuzky

Toto kolo vyhral pocitac

Celkove skore: hrac 0 – pocitac 1

Zadejte svuj tah: papir

Hrac zahral Papir, pocitac zahral Kamen

Toto kolo vyhral hrac

Celkove skore: hrac 1 – pocitac 1

Zadejte svuj tah: nuzky

Hrac zahral Nuzky, pocitac zahral Papir

Toto kolo vyhral hrac

Celkove skore: hrac 2 – pocitac 1

Celou hru vyhral hrac

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
#include <string>
#include <optional>
#include <random>
#include <thread>
#include <chrono>
#include <cctype>
#include <stdexcept>

// Возможные ходы и результаты раунда представлены перечислениями.
enum class Tah { Kamen, Nuzky, Papir };
enum class Vysledek { Nerozhodne, Hrac, Pocitac };
static int random_move() {
    static std::mt19937 rng(std::random_device{}());
    return std::uniform_int_distribution<int>(0,2)(rng);
}
static std::string read_line() {
    std::string value;
    if (!std::getline(std::cin,value)) throw std::runtime_error("End of input");
    auto begin=value.find_first_not_of(" \t\r\n");
    if (begin==std::string::npos) return "";
    value=value.substr(begin,value.find_last_not_of(" \t\r\n")-begin+1);
    std::size_t i=0;
    while (i < value.size()) { value[i]=static_cast<char>(std::tolower(static_cast<unsigned char>(value[i]))); ++i; }
    return value;
}
static void pause(int ms) { std::this_thread::sleep_for(std::chrono::milliseconds(ms)); }
// ANSI-последовательности очищают экран и перемещают курсор в левый верхний угол.
static void vycisti_obrazovku() { std::cout << "\x1B[2J\x1B[1;1H" << std::flush; }
static std::string tah_na_text(Tah tah) {
    if (tah==Tah::Kamen) return "Kamen";
    if (tah==Tah::Nuzky) return "Nuzky";
    return "Papir";
}
static std::string tah_emoji(Tah tah) {
    if (tah==Tah::Kamen) return "🪨";
    if (tah==Tah::Nuzky) return "✂️";
    return "📄";
}
// Многострочные рисунки соответствуют исходной Rust-версии.
static std::string tah_ascii(Tah tah) {
    if (tah==Tah::Kamen) return "\n    _______\n---'   ____)\n      (_____)\n      (_____)\n      (____)\n---.__(___)\n";
    if (tah==Tah::Nuzky) return "\n    _______\n---'   ____)____\n          ______)\n       __________)\n      (____)\n---.__(___)\n";
    return "\n    _______\n---'   ____)____\n          ______)\n          _______)\n         _______)\n---.__________)\n";
}
// Три кадра с той же паузой 600 мс, что в Rust.
static void animace() {
    vycisti_obrazovku(); std::cout << '\n'; std::cout << "             KAMEN!" << '\n'; std::cout << '\n'; std::cout << tah_ascii(Tah::Kamen) << '\n'; pause(600);
    vycisti_obrazovku(); std::cout << '\n'; std::cout << "             NUZKY!" << '\n'; std::cout << '\n'; std::cout << tah_ascii(Tah::Nuzky) << '\n'; pause(600);
    vycisti_obrazovku(); std::cout << '\n'; std::cout << "             PAPIR!" << '\n'; std::cout << '\n'; std::cout << tah_ascii(Tah::Papir) << '\n'; pause(600);
}
// Неизвестное слово даёт отсутствующий ход; вызывающая функция повторяет запрос.
static std::optional<Tah> nacti_tah_hrace() {
    std::cout << "Zadejte svuj tah [kamen / nuzky / papir]: "; std::cout.flush();
    std::string vstup=read_line();
    if (vstup == "kamen" || vstup == "k") return Tah::Kamen;
    if (vstup == "nuzky" || vstup == "n") return Tah::Nuzky;
    if (vstup == "papir" || vstup == "p") return Tah::Papir;
    return std::nullopt;
}
// Случайное целое 0, 1 или 2 определяет ход компьютера.
static Tah tah_pocitace() {
    int cislo=random_move();
    if (cislo==0) return Tah::Kamen;
    if (cislo==1) return Tah::Nuzky;
    return Tah::Papir;
}
static Vysledek vysledek_kola(Tah hrac,Tah pocitac) {
    if (hrac==pocitac) return Vysledek::Nerozhodne;
    if ((hrac==Tah::Kamen && pocitac==Tah::Nuzky) || (hrac==Tah::Nuzky && pocitac==Tah::Papir) || (hrac==Tah::Papir && pocitac==Tah::Kamen)) return Vysledek::Hrac;
    return Vysledek::Pocitac;
}
static void zobraz_skore(int skore_hrac,int skore_pocitac) {
    std::cout << '\n'; std::cout << "========================================" << '\n';
    std::cout << "        HRAC " << skore_hrac << " : " << skore_pocitac << " POCITAC" << '\n';
    std::cout << "========================================" << '\n'; std::cout << '\n';
}
static void zobraz_tahy(Tah hrac,Tah pocitac) {
    std::cout << '\n'; std::cout << "HRAC                         POCITAC" << '\n';
    std::cout << tah_emoji(hrac) << " " << tah_na_text(hrac) << "                     " << tah_na_text(pocitac) << " " << tah_emoji(pocitac) << '\n';
    std::cout << '\n'; std::cout << "HRAC:" << '\n'; std::cout << tah_ascii(hrac) << '\n'; std::cout << "POCITAC:" << '\n'; std::cout << tah_ascii(pocitac) << '\n';
}
static void zobraz_uvod() {
    vycisti_obrazovku(); std::cout << "========================================" << '\n'; std::cout << "         KAMEN - NUZKY - PAPIR" << '\n';
    std::cout << "========================================" << '\n'; std::cout << '\n'; std::cout << "Hrajeme na dve vitezna kola." << '\n'; std::cout << '\n';
    std::cout << "Muzete zadat:" << '\n'; std::cout << "  kamen / k" << '\n'; std::cout << "  nuzky / n" << '\n'; std::cout << "  papir / p" << '\n'; std::cout << '\n';
}
// Ничья не меняет счёт. Игра заканчивается после двух побед одной стороны.
static void hraj_hru() {
    int skore_hrac=0,skore_pocitac=0;
    zobraz_uvod();
    while (skore_hrac < 2 && skore_pocitac < 2) {
        Tah hrac;
        while (true) {
            std::optional<Tah> tah=nacti_tah_hrace();
            if (tah.has_value()) { hrac=tah.value(); break; }
            std::cout << '\n'; std::cout << "Tento tah je neplatny." << '\n'; std::cout << '\n';
        }
        Tah pocitac=tah_pocitace();
        animace(); vycisti_obrazovku(); zobraz_tahy(hrac,pocitac);
        Vysledek vysledek=vysledek_kola(hrac,pocitac);
        if (vysledek==Vysledek::Nerozhodne) std::cout << "Toto kolo je nerozhodne." << '\n';
        else if (vysledek==Vysledek::Hrac) { std::cout << "Toto kolo vyhral hrac." << '\n'; skore_hrac += 1; }
        else { std::cout << "Toto kolo vyhral pocitac." << '\n'; skore_pocitac += 1; }
        zobraz_skore(skore_hrac,skore_pocitac);
        if (skore_hrac < 2 && skore_pocitac < 2) pause(1200);
    }
    std::cout << '\n';
    if (skore_hrac==2) std::cout << "CELKOVYM VITEZEM JE HRAC!" << '\n';
    else std::cout << "CELKOVYM VITEZEM JE POCITAC!" << '\n';
    std::cout << '\n';
}
static bool chce_hrat_znovu() {
    std::cout << "Hrat znovu? [a/n]: "; std::cout.flush();
    std::string vstup=read_line();
    return vstup == "a" || vstup == "ano";
}
int main() {
    std::cout << std::boolalpha;
    while (true) {
        hraj_hru();
        if (!chce_hrat_znovu()) break;
    }
    std::cout << '\n'; std::cout << "Diky za hru." << '\n';
}
