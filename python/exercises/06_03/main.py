"""
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
"""

MAX_LENGTH = 100


def compare(first, second):
    i = 0
    while i < len(first) and i < len(second):
        if first[i] < second[i]:
            return -1
        if first[i] > second[i]:
            return 1
        i += 1
    if len(first) < len(second):
        return -1
    if len(first) > len(second):
        return 1
    return 0


def index_of(array, search, start):
    if start < 0 or start > len(array):
        return -1
    if len(search) == 0:
        return start
    i = start
    while i <= len(array) - len(search):
        j = 0
        while j < len(search) and array[i + j] == search[j]:
            j += 1
        if j == len(search):
            return i
        i += 1
    return -1


def substring(array, start, length):
    if start < 0 or length < 0 or start + length > len(array) or length > MAX_LENGTH:
        raise ValueError("Invalid substring bounds")
    result = ['\0'] * length
    i = 0
    while i < length:
        result[i] = array[start + i]
        i += 1
    return result


def insert(array, index, insert_array):
    if index < 0 or index > len(array) or len(array) + len(insert_array) > MAX_LENGTH:
        raise ValueError("Invalid insertion bounds")
    result = ['\0'] * (len(array) + len(insert_array))
    i = 0
    while i < len(result):
        if i < index:
            result[i] = array[i]
        elif i < index + len(insert_array):
            result[i] = insert_array[i - index]
        else:
            result[i] = array[i - len(insert_array)]
        i += 1
    return result


def remove(array, index, length):
    if index < 0 or length < 0 or index + length > len(array) or len(array) - length > MAX_LENGTH:
        raise ValueError("Invalid removal bounds")
    result = ['\0'] * (len(array) - length)
    i = 0
    while i < len(result):
        result[i] = array[i] if i < index else array[i + length]
        i += 1
    return result


def replace(array, search, replacement):
    if len(search) == 0:
        raise ValueError("Search array cannot be empty")
    work = ['\0'] * MAX_LENGTH
    i = position = 0
    while i < len(array):
        j = 0
        while j < len(search) and i + j < len(array) and array[i + j] == search[j]:
            j += 1
        if j == len(search):
            j = 0
            while j < len(replacement):
                if position >= MAX_LENGTH:
                    raise ValueError("Result is too long")
                work[position] = replacement[j]
                position += 1
                j += 1
            i += len(search)
        else:
            if position >= MAX_LENGTH:
                raise ValueError("Result is too long")
            work[position] = array[i]
            position += 1
            i += 1
    return substring(work, 0, position)


def print_chars(array):
    i = 0
    while i < len(array):
        print(array[i], end="")
        i += 1
    print()


def main():
    first, second = list("klokan"), list("klobouk")
    print(f"A) Compare: {compare(first, second)}")
    print(f"B) IndexOf 'o' from position 3: {index_of(second, ['o'], 3)}")
    third = substring(second, 2, 3)
    print("C) Substring: ", end="")
    print_chars(third)
    print("D) Insert: ", end="")
    print_chars(insert(first, 1, third))
    fifth = remove(second, 3, 2)
    print("E) Remove: ", end="")
    print_chars(fifth)
    print("F) Replace: ", end="")
    print_chars(replace(fifth, ['k'], third))

if __name__ == "__main__":
    main()
