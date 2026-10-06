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

using System;
using System.Globalization;
using System.Numerics;

// Отдельная программа: алгоритм соответствует одноимённому Rust-заданию.
internal static class Program
{

    // Возможные ходы и результаты раунда представлены перечислениями.
    enum Tah { Kamen, Nuzky, Papir }
    enum Vysledek { Nerozhodne, Hrac, Pocitac }
    private static readonly Random RNG=new Random();
    static int random_move() { return RNG.Next(0,3); }
    static string read_line() { return (Console.ReadLine() ?? throw new InvalidOperationException("End of input")).Trim().ToLowerInvariant(); }
    static void pause(int ms) { System.Threading.Thread.Sleep(ms); }
    // ANSI-последовательности очищают экран и перемещают курсор в левый верхний угол.
    static void vycisti_obrazovku() { Console.Write("\u001b[2J\u001b[1;1H"); Console.Out.Flush(); }
    static string tah_na_text(Tah tah) {
        if (tah==Tah.Kamen) return "Kamen";
        if (tah==Tah.Nuzky) return "Nuzky";
        return "Papir";
    }
    static string tah_emoji(Tah tah) {
        if (tah==Tah.Kamen) return "🪨";
        if (tah==Tah.Nuzky) return "✂️";
        return "📄";
    }
    // Многострочные рисунки соответствуют исходной Rust-версии.
    static string tah_ascii(Tah tah) {
        if (tah==Tah.Kamen) return "\n    _______\n---'   ____)\n      (_____)\n      (_____)\n      (____)\n---.__(___)\n";
        if (tah==Tah.Nuzky) return "\n    _______\n---'   ____)____\n          ______)\n       __________)\n      (____)\n---.__(___)\n";
        return "\n    _______\n---'   ____)____\n          ______)\n          _______)\n         _______)\n---.__________)\n";
    }
    // Три кадра с той же паузой 600 мс, что в Rust.
    static void animace() {
        vycisti_obrazovku(); Console.WriteLine(""); Console.WriteLine("             KAMEN!"); Console.WriteLine(""); Console.WriteLine("{0}", tah_ascii(Tah.Kamen)); pause(600);
        vycisti_obrazovku(); Console.WriteLine(""); Console.WriteLine("             NUZKY!"); Console.WriteLine(""); Console.WriteLine("{0}", tah_ascii(Tah.Nuzky)); pause(600);
        vycisti_obrazovku(); Console.WriteLine(""); Console.WriteLine("             PAPIR!"); Console.WriteLine(""); Console.WriteLine("{0}", tah_ascii(Tah.Papir)); pause(600);
    }
    // Неизвестное слово даёт отсутствующий ход; вызывающая функция повторяет запрос.
    static Tah? nacti_tah_hrace() {
        Console.Write("Zadejte svuj tah [kamen / nuzky / papir]: "); Console.Out.Flush();
        string vstup=read_line();
        if (vstup == "kamen" || vstup == "k") return Tah.Kamen;
        if (vstup == "nuzky" || vstup == "n") return Tah.Nuzky;
        if (vstup == "papir" || vstup == "p") return Tah.Papir;
        return null;
    }
    // Случайное целое 0, 1 или 2 определяет ход компьютера.
    static Tah tah_pocitace() {
        int cislo=random_move();
        if (cislo==0) return Tah.Kamen;
        if (cislo==1) return Tah.Nuzky;
        return Tah.Papir;
    }
    static Vysledek vysledek_kola(Tah hrac,Tah pocitac) {
        if (hrac==pocitac) return Vysledek.Nerozhodne;
        if ((hrac==Tah.Kamen && pocitac==Tah.Nuzky) || (hrac==Tah.Nuzky && pocitac==Tah.Papir) || (hrac==Tah.Papir && pocitac==Tah.Kamen)) return Vysledek.Hrac;
        return Vysledek.Pocitac;
    }
    static void zobraz_skore(int skore_hrac,int skore_pocitac) {
        Console.WriteLine(""); Console.WriteLine("========================================");
        Console.WriteLine("        HRAC {0} : {1} POCITAC", skore_hrac, skore_pocitac);
        Console.WriteLine("========================================"); Console.WriteLine("");
    }
    static void zobraz_tahy(Tah hrac,Tah pocitac) {
        Console.WriteLine(""); Console.WriteLine("HRAC                         POCITAC");
        Console.WriteLine("{0} {1}                     {2} {3}", tah_emoji(hrac), tah_na_text(hrac), tah_na_text(pocitac), tah_emoji(pocitac));
        Console.WriteLine(""); Console.WriteLine("HRAC:"); Console.WriteLine("{0}", tah_ascii(hrac)); Console.WriteLine("POCITAC:"); Console.WriteLine("{0}", tah_ascii(pocitac));
    }
    static void zobraz_uvod() {
        vycisti_obrazovku(); Console.WriteLine("========================================"); Console.WriteLine("         KAMEN - NUZKY - PAPIR");
        Console.WriteLine("========================================"); Console.WriteLine(""); Console.WriteLine("Hrajeme na dve vitezna kola."); Console.WriteLine("");
        Console.WriteLine("Muzete zadat:"); Console.WriteLine("  kamen / k"); Console.WriteLine("  nuzky / n"); Console.WriteLine("  papir / p"); Console.WriteLine("");
    }
    // Ничья не меняет счёт. Игра заканчивается после двух побед одной стороны.
    static void hraj_hru() {
        int skore_hrac=0,skore_pocitac=0;
        zobraz_uvod();
        while (skore_hrac < 2 && skore_pocitac < 2) {
            Tah hrac;
            while (true) {
                Tah? tah=nacti_tah_hrace();
                if (tah.HasValue) { hrac=tah.Value; break; }
                Console.WriteLine(""); Console.WriteLine("Tento tah je neplatny."); Console.WriteLine("");
            }
            Tah pocitac=tah_pocitace();
            animace(); vycisti_obrazovku(); zobraz_tahy(hrac,pocitac);
            Vysledek vysledek=vysledek_kola(hrac,pocitac);
            if (vysledek==Vysledek.Nerozhodne) Console.WriteLine("Toto kolo je nerozhodne.");
            else if (vysledek==Vysledek.Hrac) { Console.WriteLine("Toto kolo vyhral hrac."); skore_hrac += 1; }
            else { Console.WriteLine("Toto kolo vyhral pocitac."); skore_pocitac += 1; }
            zobraz_skore(skore_hrac,skore_pocitac);
            if (skore_hrac < 2 && skore_pocitac < 2) pause(1200);
        }
        Console.WriteLine("");
        if (skore_hrac==2) Console.WriteLine("CELKOVYM VITEZEM JE HRAC!");
        else Console.WriteLine("CELKOVYM VITEZEM JE POCITAC!");
        Console.WriteLine("");
    }
    static bool chce_hrat_znovu() {
        Console.Write("Hrat znovu? [a/n]: "); Console.Out.Flush();
        string vstup=read_line();
        return vstup == "a" || vstup == "ano";
    }
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        while (true) {
            hraj_hru();
            if (!chce_hrat_znovu()) break;
        }
        Console.WriteLine(""); Console.WriteLine("Diky za hru.");
    }
}
