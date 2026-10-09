/*
Cvičení 03 — Úloha 3.2
Název: BMI
Body: 1 bod; 1 bod (v pořadí podúloh)

Zadání:
BMI (Body Mass Index) se počítá: váha v kilogramech vydělená druhou mocninou výšky v metrech.

a) Napište funkci, která pro zadanou hodnotu výšky a váhy spočte BMI. [1 bod]

b) Napište funkci, která místo číselné hodnoty bude vracet výčtový typ. [1 bod]

BMI                výčtový typ

[0; 16,5)          těžká podvýživa

[16,5; 18,5)     podváha

[18,5; 25)        optimální váha

[25; 30)           nadváha

[30; 35)           obezita prvního stupně

[35; 40)           obezita druhého stupně

[40; oo)           obezita třetího stupně

Ukázka:

x = SpoctiBMI1(1.70, 61.5); // 21.28

y = SpoctiBMI2(1.67, 80); // Nadvaha

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

    enum BmiType { SevereUnderweight, Underweight, NormalWeight, Overweight, ObesityClassOne, ObesityClassTwo, ObesityClassThree }
    // Рост задаётся в метрах, вес — в килограммах. Рост должен быть положительным.
    static double CalculateBmi(double height, double weight) { return weight / (height * height); }
    // Проверяем верхние границы по порядку: нижние уже исключены предыдущими ветками.
    static BmiType ClassifyBmi(double height, double weight) {
        double bmi = weight / (height * height);
        if (bmi < 16.5) return BmiType.SevereUnderweight;
        else if (bmi < 18.5) return BmiType.Underweight;
        else if (bmi < 25.0) return BmiType.NormalWeight;
        else if (bmi < 30.0) return BmiType.Overweight;
        else if (bmi < 35.0) return BmiType.ObesityClassOne;
        else if (bmi < 40.0) return BmiType.ObesityClassTwo;
        else return BmiType.ObesityClassThree;
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void Main() {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        double x = CalculateBmi(1.70, 61.5);
        BmiType y = ClassifyBmi(1.67, 80.0);
        Console.WriteLine("BMI is {0:F2}.", x);
        // Переводим категорию в текст для пользователя.
        if (y == BmiType.SevereUnderweight) Console.WriteLine("BMI category: severe underweight.");
        else if (y == BmiType.Underweight) Console.WriteLine("BMI category: underweight.");
        else if (y == BmiType.NormalWeight) Console.WriteLine("BMI category: normal weight.");
        else if (y == BmiType.Overweight) Console.WriteLine("BMI category: overweight.");
        else if (y == BmiType.ObesityClassOne) Console.WriteLine("BMI category: class I obesity.");
        else if (y == BmiType.ObesityClassTwo) Console.WriteLine("BMI category: class II obesity.");
        else Console.WriteLine("BMI category: class III obesity.");
    }
}
