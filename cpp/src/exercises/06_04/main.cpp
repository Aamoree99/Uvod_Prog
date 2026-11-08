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

#include <cstdint>
#include <initializer_list>
#include <iostream>
#include <limits>
#include <memory>
#include <random>
#include <stdexcept>
#include <string>

// Обычный динамический массив и его длина. У записи нет методов;
// shared_ptr управляет временем жизни выделенного массива при возврате из функций.
template<class T> struct Array {
    std::shared_ptr<T[]> data;
    int length = 0;
};
template<class T> Array<T> makeArray(int length) {
    if (length < 0) throw std::invalid_argument("Negative array length");
    return {std::shared_ptr<T[]>(new T[length]{}), length};
}
template<class T> Array<T> arrayOf(std::initializer_list<T> values) {
    Array<T> result = makeArray<T>(static_cast<int>(values.size()));
    const T* source = values.begin();
    for (int i = 0; i < result.length; i++) result.data[i] = source[i];
    return result;
}
std::mt19937 randomEngine(std::random_device{}());
int randomInt(int bound) {
    return std::uniform_int_distribution<int>(0, bound - 1)(randomEngine);
}
std::string lowerText(std::string text) {
    for (int i = 0; i < static_cast<int>(text.size()); i++)
        if (text[i] >= 'A' && text[i] <= 'Z') text[i] = static_cast<char>(text[i] + 32);
    return text;
}

static bool wins(int kind, int number, int result);
static int multiplier(int kind);
static int parseKind(std::string token);
static bool quit(std::string token);
static int skipSpace(std::string text, int p);
static int tokenEnd(std::string text, int p);
static std::int64_t parseAmount(std::string token);
static bool canAdd(std::int64_t a, std::int64_t b);

static bool wins(int kind, int number, int result) {
    if (kind == 0) return result == number;
    if (result == 0) return false;
    if (kind == 1) return result % 2 == 1;
    if (kind == 2) return result % 2 == 0;
    if (kind == 3) return result <= 18;
    if (kind == 4) return result >= 19;
    return result >= (kind - 5) * 12 + 1 && result <= (kind - 4) * 12;
}
static int multiplier(int kind) { return kind == 0 ? 35 : (kind < 5 ? 1 : 2); }
static int parseKind(std::string token) {
    Array<std::string> names = arrayOf<std::string>({"odd", "even", "low", "high", "dozen1", "dozen2", "dozen3", "licha", "suda", "nizka", "vysoka"});
    Array<int> kinds = arrayOf<int>({1, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4});
    for (int i = 0; i < names.length; i++)
        if ((token == names.data[i])) return kinds.data[i];
    return -1;
}
static bool quit(std::string token) { return (token == "quit") || (token == "exit") || (token == "konec"); }
// Токенизация по индексам; не используем коллекции и foreach.
static int skipSpace(std::string text, int p) {
    while (p < static_cast<int>(text.size()) && text[p] <= ' ') p++;
    return p;
}
static int tokenEnd(std::string text, int p) {
    while (p < static_cast<int>(text.size()) && text[p] > ' ') p++;
    return p;
}
static std::int64_t parseAmount(std::string token) {
    if (static_cast<int>(token.size()) == 0) return -1;
    std::int64_t value = 0;
    for (int i = 0; i < static_cast<int>(token.size()); i++) {
        int digit = token[i] - '0';
        if (digit < 0 || digit > 9 || value > (std::numeric_limits<std::int64_t>::max() - digit) / 10) return -1;
        value = value * 10 + digit;
    }
    return value;
}
static bool canAdd(std::int64_t a, std::int64_t b) {
    return !(b > 0 && a > std::numeric_limits<std::int64_t>::max() - b || b < 0 && a < std::numeric_limits<std::int64_t>::min() - b);
}
int main() {

        std::int64_t balance = 0;
    std::cout << ("EUROPEAN ROULETTE\nTYPE AMOUNT [TYPE AMOUNT ...], done, quit") << '\n';
    std::cout << ("Types: 0..36, odd/even/low/high, dozen1/dozen2/dozen3 (licha/suda/nizka/vysoka)") << '\n';
    bool finished = false;
    while (!finished) {
        std::cout << ("Current balance: ") << (balance) << '\n';
        std::string text;
        if (!std::getline(std::cin, text)) break;
        if (static_cast<int>(text.size()) > 4096) { std::cout << ("Input too std::int64_t") << '\n'; continue; }
        Array<int> kinds = makeArray<int>(100), numbers = makeArray<int>(100);
        Array<std::int64_t> amounts = makeArray<std::int64_t>(100);
        int count = 0, p = 0;
        while (p < static_cast<int>(text.size())) {
            p = skipSpace(text, p);
            int end = tokenEnd(text, p);
            if (p == end) break;
            std::string token = lowerText(text.substr(p, (end) - (p)));
            p = end;
            if (quit(token)) { finished = true; break; }
            if ((token == "done") || (token == "hotovo")) break;
            int kind = parseKind(token), number = 0;
            if (kind < 0) {
                std::int64_t parsed = parseAmount(token);
                if (parsed < 0 || parsed > 36) continue;
                kind = 0;
                number = (int)parsed;
            }
            p = skipSpace(text, p);
            end = tokenEnd(text, p);
            token = lowerText(text.substr(p, (end) - (p)));
            p = end;
            if (quit(token)) { finished = true; break; }
            std::int64_t amount = parseAmount(token);
            if (amount <= 0) continue;
            if (count == 100) { std::cout << ("Maximum 100 bets") << '\n'; break; }
            kinds.data[count] = kind;
            numbers.data[count] = number;
            amounts.data[count++] = amount;
        }
        if (finished) break;
        if (count == 0) { std::cout << ("No valid bets") << '\n'; continue; }
        int result = randomInt(37);
        std::cout << ("Roulette number: ") << (result) << '\n';
        std::int64_t change = 0;
        for (int i = 0; i < count; i++) {
            bool won = wins(kinds.data[i], numbers.data[i], result);
            int factor = multiplier(kinds.data[i]);
            if (won && amounts.data[i] > std::numeric_limits<std::int64_t>::max() / factor) { std::cout << ("PAYOUT OVERFLOW") << '\n'; continue; }
            std::int64_t profit = won ? amounts.data[i] * factor : -amounts.data[i];
            if (!canAdd(change, profit)) { std::cout << ("BALANCE OVERFLOW") << '\n'; continue; }
            change += profit;
            std::cout << ("Kind: ") << (kinds.data[i]) << (" Number: ") << (numbers.data[i]) << (" Amount: ") << (amounts.data[i]) << (" | ") << ((won ? "WIN " : "LOSE ")) << (profit) << '\n';
        }
        if (!canAdd(balance, change)) { std::cout << ("BALANCE OVERFLOW") << '\n'; break; }
        balance += change;
        std::cout << ("Round result: ") << (change) << '\n';
    }
    std::cout << ("Final balance: ") << (balance) << '\n';
}
