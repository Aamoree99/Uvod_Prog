/*
Cvičení 04 — Úloha 4.8
Název: Zaplatit mincemi
Body: 4 body; 2 body (v pořadí podúloh)

Zadání:
a) Napište funkci, která pro zadané pole obsahující sestupně setříděné nominální hodnoty mincí
platících v ČR spočte minimální počet kusů mincí, které je nutno použít pro zaplacení zadané
částky.  [4 body].

b) Najděte sadu mincí (jinou než v ukázce) a částku, pro kterou funkce nenajde optimální řešení.
Nalezená sada pravděpodobně nebude platit v žádné zemi na světě. [2 body]


Ukázka:

int[] mince = { 50, 20, 10, 5, 2, 1 };

pocet = ZaplatitMincemi(mince, 97); // 5

pocet = ZaplatitMincemi(mince, 88); // 6

pocet = ZaplatitMincemi(mince, 26); // 3

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

internal static class Program
{
    // Номиналы положительны, отсортированы по убыванию и включают единицу.
    // Возвращаем число монет, выбирая сначала самые крупные.
    static int PayWithCoins(int[] coins, int amount)
    {
        int remainder = amount, count = 0, i = 0;
        while (remainder > 0)
        {
            int coinCount = remainder / coins[i];
            count += coinCount;
            remainder -= coinCount * coins[i];
            i += 1;
        }
        return count;
    }
    // Перебираем количества каждого номинала; index указывает текущую монету.
    // Большое число обозначает невозможность собрать оставшуюся сумму.
    static int OptimalCoinCount(int[] coins, int index, int amount)
    {
        if (amount == 0) return 0;
        if (index >= coins.Length) return int.MaxValue / 2;
        int coin = coins[index], count = amount / coin, minimum = int.MaxValue / 2;
        while (count >= 0)
        {
            int remainder = amount - count * coin;
            int remainingCount = OptimalCoinCount(coins, index + 1, remainder);
            if (remainingCount < int.MaxValue / 2)
            {
                int total = count + remainingCount;
                if (total < minimum) minimum = total;
            }
            count -= 1;
        }
        return minimum;
    }
    // Запускаем пример из упражнения и выводим результаты.
    public static void Main()
    {
        int[] coins = {50, 20, 10, 5, 2, 1};
        Console.WriteLine("PART A\n");
        Console.WriteLine("The minimum number of coins needed to pay {0} CZK is {1}.", 97, PayWithCoins(coins, 97));
        Console.WriteLine("The minimum number of coins needed to pay {0} CZK is {1}.", 88, PayWithCoins(coins, 88));
        Console.WriteLine("The minimum number of coins needed to pay {0} CZK is {1}.", 26, PayWithCoins(coins, 26));
        // Для суммы 6 жадный выбор даёт 4+1+1, а оптимальный — 3+3.
        int[] strangeCoins = {4, 3, 1};
        int strangeAmount = 6;
        Console.WriteLine("\nPART B\n");
        Console.WriteLine("Coin set: [" + string.Join(", ", strangeCoins) + "]");
        Console.WriteLine("Amount: {0}", strangeAmount);
        Console.WriteLine("Greedy algorithm: {0} coins", PayWithCoins(strangeCoins, strangeAmount));
        Console.WriteLine("Optimal solution: {0} coins", OptimalCoinCount(strangeCoins, 0, strangeAmount));
    }
}
