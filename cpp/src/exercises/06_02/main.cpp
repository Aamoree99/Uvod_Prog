/*
Cvičení 06 — Úloha 6.2
Název: Césarova šifra
Body: 1 bod; 2 body (v pořadí podúloh)

Zadání:
Césarova šifra je jednou z nejstarších metod pro převod přenášené zprávy do zašifrované podoby,
která je nečitelná pro neinformovaného útočníka. Parametrem šifry je celé číslo nazvané posun.
Každé písmeno zprávy je zaměněno za písmeno, které se v abecedě nachází za tímto písmenem o počet
znaků určený hodnotou parametru posun. Původní zprávu získáme aplikací tohoto postupu na
zašifrovanou zprávu s modifikovaným parametrem posun. Hodnota parametru posun bude mít oproti
původní hodnotě tohoto parametru opačné znaménko.



a) Implementujte funkci pro zašifrování textového řetězce pomocí této šifry. Pro jednoduchost
nezkoumejte, co jsou písmena a posouvejte celý řetězec. [1 bod].

b) Slovně (pomocí komentáře ve zdrojovém kódu) popište postup, kterým byste v roli útočníka bez
znalosti parametru posun ze zašifrované zprávy získali původní zprávu. [2 body]


Ukázka:

string s = "Toto je tajny text: 1726";

string zasifrovane = CezarovaSifra(s, 20);

// h���4~y4�u~‚�4�yŒ�N4EKFJ

string odsifrovane = CezarovaSifra(zasifrovane, -20);

// Musíme získat původní text

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
#include <memory>
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

void printUtf16(const char* label, const std::u16string& text) {
    std::cout << label;
    for (int i = 0; i < static_cast<int>(text.size()); i++) {
        unsigned int c = text[i];
        if (c < 128) std::cout << static_cast<char>(c);
        else if (c < 2048) {
            std::cout << static_cast<char>(0xC0 | (c >> 6));
            std::cout << static_cast<char>(0x80 | (c & 63));
        } else {
            std::cout << static_cast<char>(0xE0 | (c >> 12));
            std::cout << static_cast<char>(0x80 | ((c >> 6) & 63));
            std::cout << static_cast<char>(0x80 | (c & 63));
        }
    }
    std::cout << '\n';
}

static std::u16string caesarCipher(std::u16string text, int shift);

// char — кодовая единица UTF-16; сдвиг циклический по модулю 65536.
static std::u16string caesarCipher(std::u16string text, int shift) {
    Array<char16_t> result = makeArray<char16_t>(static_cast<int>(text.size()));
    int normalized = shift % 65536;
    for (int i = 0; i < static_cast<int>(text.size()); i++)
        result.data[i] = (char16_t)((text[i] + normalized + 65536) % 65536);
    return std::u16string(result.data.get(), result.length);
}
int main() {
    std::u16string text = u"Toto je tajny text: 1726";
    std::u16string encrypted = caesarCipher(text, 20);
    std::u16string decrypted = caesarCipher(encrypted, -20);
    printUtf16("Original: ", text);
    printUtf16("Encrypted: ", encrypted);
    printUtf16("Decrypted: ", decrypted);
    std::cout << ("Round trip: ") << ((text == decrypted)) << '\n';
}
