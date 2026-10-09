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

// --------------------------------------------------
// B) POPIS PROLOMENÍ CAESAROVY ŠIFRY
// --------------------------------------------------

// ČESKY:
//
// Caesarovu šifru lze prolomit metodou hrubé síly
// (brute force), protože šifra používá pouze jeden
// parametr — celočíselný posun.
//
// Útočník postupně vyzkouší všechny možné hodnoty
// posunu a pro každou hodnotu dešifruje zprávu
// pomocí opačného posunu.
//
// U každého výsledku zkontroluje, zda obsahuje
// srozumitelný text, známá slova a správnou gramatiku.
//
// Správný posun lze určit také pomocí frekvenční
// analýzy znaků. Například porovnáním četnosti
// písmen ve zprávě s četností písmen v daném jazyce.
//
// Caesarova šifra proto není bezpečná pro ochranu
// důvěrných informací.

// ПО-РУССКИ:
//
// Шифр Цезаря можно взломать полным перебором
// (brute force), поскольку он использует только
// один параметр — целочисленный сдвиг.
//
// Злоумышленник последовательно пробует все
// возможные значения сдвига и расшифровывает
// сообщение с противоположным знаком сдвига.
//
// Затем проверяет каждый результат: появились ли
// понятные слова, осмысленные предложения
// и правильные грамматические конструкции.
//
// Также можно применять частотный анализ:
// сравнивать частоту символов в расшифрованном
// сообщении с частотой букв обычного языка.
//
// Поэтому шифр Цезаря не обеспечивает
// надёжную защиту конфиденциальных данных.

// --------------------------------------------------
// A) CAESAR CIPHER
// --------------------------------------------------

// Шифруем или расшифровываем строку.
//
// text — исходный текст.
// shift — целочисленный сдвиг.
//
// Положительный shift шифрует текст.
// Отрицательный shift выполняет обратный сдвиг.
//
// Обрабатываем все символы, включая пробелы,
// цифры и знаки препинания.
//
// Для совместимости с принципом C# работаем
// с 16-битными кодовыми единицами UTF-16.
//
// Возвращаем Vec<u16>, потому что результат
// произвольного сдвига не всегда является
// допустимой строкой UTF-8.
fn caesar_cipher(text: &[u16], shift: i32) -> Vec<u16> {
    // Создаём массив для результата.
    let mut result = Vec::new();

    let mut i = 0;

    while i < text.len() {
        // Получаем числовой код текущего символа.
        let character = text[i] as i32;

        // Сдвигаем код и оставляем в диапазоне
        // от 0 до 65535 (16 бит).
        //
        // rem_euclid корректно работает
        // с отрицательными сдвигами.
        let shifted = (character + shift).rem_euclid(65536);

        // Добавляем преобразованный код в результат.
        result.push(shifted as u16);

        i += 1;
    }

    result
}

// Проверяем, совпадают ли два массива кодов.
//
// Используем обычный while вместо коллекционных
// методов и итераторов.
fn arrays_equal(first: &[u16], second: &[u16]) -> bool {
    if first.len() != second.len() {
        return false;
    }

    let mut i = 0;

    while i < first.len() {
        if first[i] != second[i] {
            return false;
        }

        i += 1;
    }

    true
}

// MAIN
fn main() {
    // Исходный текст из задания.
    let text = "Toto je tajny text: 1726";

    // Преобразуем строку в кодовые единицы UTF-16.
    let original: Vec<u16> = text.encode_utf16().collect();

    // Сдвиг из примера преподавателя.
    let shift = 20;

    // Шифруем исходный текст.
    let encrypted = caesar_cipher(&original, shift);

    // Расшифровываем обратным сдвигом.
    let decrypted = caesar_cipher(&encrypted, -shift);

    // Представляем результаты для вывода.
    // from_utf16_lossy заменяет недопустимые
    // последовательности кодов символом �.
    //
    // Это влияет только на отображение,
    // не на зашифрованные данные.
    let encrypted_text = String::from_utf16_lossy(&encrypted);
    let decrypted_text = String::from_utf16_lossy(&decrypted);

    println!("CAESAR CIPHER");
    println!("========================================");

    println!("Original text:  {:?}", text);
    println!("Shift:          {}", shift);

    println!();
    println!("Encrypted text: {:?}", encrypted_text);
    println!("Decrypted text: {:?}", decrypted_text);

    println!();
    println!(
        "Decryption successful: {}",
        arrays_equal(&original, &decrypted)
    );
}
