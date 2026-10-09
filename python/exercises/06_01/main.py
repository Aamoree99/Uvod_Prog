"""
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
"""

def is_letter(character):
    return 'A' <= character <= 'Z' or 'a' <= character <= 'z'


def to_lower(character):
    if 'A' <= character <= 'Z':
        return chr(ord(character) + 32)
    return character


def next_word_bounds(text, position):
    # Сканируем исходную строку по индексам без промежуточных копий слов.
    while position < len(text) and not is_letter(text[position]):
        position += 1
    start = position
    while position < len(text) and is_letter(text[position]):
        position += 1
    return start, position


def ranges_equal(text, a_start, a_end, b_start, b_end):
    if a_end - a_start != b_end - b_start:
        return False
    i = 0
    while a_start + i < a_end:
        if to_lower(text[a_start + i]) != to_lower(text[b_start + i]):
            return False
        i += 1
    return True


def range_is_greater(text, a_start, a_end, b_start, b_end):
    i = 0
    while a_start + i < a_end and b_start + i < b_end:
        a, b = to_lower(text[a_start + i]), to_lower(text[b_start + i])
        if a != b:
            return a > b
        i += 1
    return a_end - a_start > b_end - b_start


def count_distinct_letters(text):
    count = 0
    i = 0
    while i < len(text):
        if is_letter(text[i]):
            found_before = False
            j = 0
            while j < i:
                if is_letter(text[j]) and to_lower(text[j]) == to_lower(text[i]):
                    found_before = True
                    break
                j += 1
            if not found_before:
                count += 1
        i += 1
    return count


def count_words(text):
    count = position = 0
    while position < len(text):
        start, position = next_word_bounds(text, position)
        if start < position:
            count += 1
    return count


def first_word(text, position):
    if position < 0 or position >= len(text):
        return ""
    if is_letter(text[position]):
        while position > 0 and is_letter(text[position - 1]):
            position -= 1
    start, end = next_word_bounds(text, position)
    # Единственная копия — возвращаемое слово; в Python нет строковых срезов-ссылок.
    return text[start:end]


def longest_word(text):
    best_start = best_end = position = 0
    while position < len(text):
        start, position = next_word_bounds(text, position)
        if position - start > best_end - best_start:
            best_start, best_end = start, position
    return text[best_start:best_end]


def greatest_word(text):
    best_start = best_end = position = 0
    while position < len(text):
        start, position = next_word_bounds(text, position)
        if range_is_greater(text, start, position, best_start, best_end):
            best_start, best_end = start, position
    return text[best_start:best_end]


def most_frequent_word(text):
    best_start = best_end = max_count = position = 0
    while position < len(text):
        start, position = next_word_bounds(text, position)
        if start == position:
            continue
        count = other_position = 0
        while other_position < len(text):
            other_start, other_position = next_word_bounds(text, other_position)
            if ranges_equal(text, start, position, other_start, other_position):
                count += 1
        if count > max_count:
            max_count, best_start, best_end = count, start, position
    return text[best_start:best_end]


def main():
    text = "Alena ma 2 ruce."
    print(f"Original text: {text!r}")
    print(f"A) Distinct letters: {count_distinct_letters(text)}")
    print(f"B) Word count: {count_words(text)}")
    print(f"C) Word at position 2: {first_word(text, 2)!r}")
    print(f"Word at last position: {first_word(text, len(text) - 1)!r}")
    print(f"D) Longest word: {longest_word(text)!r}")
    print(f"E) Greatest word: {greatest_word(text)!r}")
    combined = "Alena ma 2 ruce.Mama ma 2 ruce a v ruce hul"
    print(f"F) Most frequent word: {most_frequent_word(combined)!r}")
    print(f"Substring test, Ja a Jana: {most_frequent_word('Ja a Jana')!r}")
    print(f"Empty string: {count_words('')}, {longest_word('')!r}, {greatest_word('')!r}, {most_frequent_word('')!r}")

if __name__ == "__main__":
    main()
