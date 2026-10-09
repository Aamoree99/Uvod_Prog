"""
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
"""

def to_units(text):
    # Кодирование служит только для представления UTF-16; шифрование ручное.
    data = text.encode("utf-16-le", errors="surrogatepass")
    units = [0] * (len(data) // 2)
    i = 0
    while i < len(units):
        units[i] = data[2 * i] + data[2 * i + 1] * 256
        i += 1
    return units


def from_units(units):
    data = bytearray(len(units) * 2)
    i = 0
    while i < len(units):
        data[2 * i] = units[i] % 256
        data[2 * i + 1] = units[i] // 256
        i += 1
    return data.decode("utf-16-le", errors="replace")


def caesar_cipher(text, shift):
    result = [0] * len(text)
    i = 0
    while i < len(text):
        result[i] = (text[i] + shift) % 65536
        i += 1
    return result


def arrays_equal(first, second):
    if len(first) != len(second):
        return False
    i = 0
    while i < len(first):
        if first[i] != second[i]:
            return False
        i += 1
    return True


def main():
    text = "Toto je tajny text: 1726"
    original = to_units(text)
    encrypted = caesar_cipher(original, 20)
    decrypted = caesar_cipher(encrypted, -20)
    print(f"Original text: {text!r}\nShift: 20")
    print(f"Encrypted text: {from_units(encrypted)!r}\nDecrypted text: {from_units(decrypted)!r}")
    print(f"Decryption successful: {arrays_equal(original, decrypted)}")

if __name__ == "__main__":
    main()
