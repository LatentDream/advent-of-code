using System.Data;
using System.Collections.Generic;
using System.Reflection.PortableExecutable;


class Program
{
    static void Main(string[] args)
    {
        Console.ForegroundColor = ConsoleColor.Green;
        Console.WriteLine("Advent of code 2023 | Day 3");
        Console.ForegroundColor = ConsoleColor.Magenta;
        Console.WriteLine("------------------------------------");
        Console.ForegroundColor = ConsoleColor.White;

        if (args.Length == 0)
        {
            Console.WriteLine("Please provide which part to run (1 or 2)");
            return;
        }  

        String part = args[0];
        Console.WriteLine("Running part " + part);
        String[] input = File.ReadAllLines("input.txt");

        // Parse the input
         int rows = input.Length;
        int columns = input[0].Length;
        char[,] map = new char[rows, columns];
        Console.ForegroundColor = ConsoleColor.Red;
        Console.WriteLine("Map size: " + rows + "x" + columns);
        Console.ForegroundColor = ConsoleColor.White;
        List<Number> numbers = new List<Number>();
        Number? current_number = null;

        // Parse the map and find the numbers
        for (int row = 0; row < rows; row++) {
            String line = input[row];
            for (int col = 0; col < columns; col++) {
                char c = line[col];
                map[row,col] = c;
                if (!char.IsDigit(c)) {
                    // Reset Number Parser
                    if (current_number != null) {
                        numbers.Add(current_number);
                        current_number = null;
                    }
                } else {
                    // Add char to number parser
                    if (current_number == null) {
                        current_number = new Number(new Position(row, col));
                    }
                    current_number.AddChar(c);
                }
            }
            // Reset Number Parser on end of line
            if (current_number != null) {
                numbers.Add(current_number);
                current_number = null;
            }
        }
        Console.WriteLine("Numbers found: " + numbers.Count);

        if (part == "1") {
            Part1(numbers, map, rows, columns);
        } else {
            Part2(numbers, map, rows, columns);
        }
    }

    static void Part1(List<Number> numbers, char[,] map, int rows, int columns) {
        // Find all the number with a symbol near by and add them
        /* * * * * * * * *
        *....XXXXX.......
        *....X123X.......
        *....XXXXX.......
        * * * * * * * * */
        int sum = 0;
        foreach (Number number in numbers) {
            bool is_valid = false;
            int col_start = number.GetPosition().col - 1;
            int col_end = number.GetPosition().col + number.GetLength() + 1;
            int row = number.GetPosition().row;
            for (int cp = col_start; cp <= col_end; cp++) {
                if (cp >= 0 && cp < columns) {
                    // Check position above
                    if (row - 1 >= 0) {
                        char c_at_ps = map[row-1, cp];
                        int row_px = row - 1;
                        if (c_at_ps != '.' && !char.IsDigit(c_at_ps)) {
                            is_valid = true;
                            break;
                        }
                    }
                    // Check position below
                    if (row + 1 < rows) {
                        char c_at_ps = map[row+1, cp];
                        int row_px = row + 1;
                        if (c_at_ps != '.' && !char.IsDigit(c_at_ps)) {
                            is_valid = true;
                            break;
                        }
                    }
                }
            }
            // Check left
            if (col_start >= 0) {
                char c_at_ps = map[row, col_start];
                if (c_at_ps != '.' && !char.IsDigit(c_at_ps)) {
                    is_valid = true;
                }
            }
            // Check right
            if (col_end < columns) {
                char c_at_ps = map[row, col_end];
                if (c_at_ps != '.' && !char.IsDigit(c_at_ps)) {
                    is_valid = true;
                }
            }
            if (is_valid) {
                sum += number.GetValue();
                // Console.WriteLine("Found valid number: " + number.GetValue());
            }

        }
        Console.WriteLine("Sum of all valid numbers: " + sum);
    }

    static void Part2(List<Number> numbers, char[,] map, int rows, int columns) {
        /* * * * * * * * *
        * Find all the * with two number near them and multiply them
        *................
        *.....123........
        *......*.........
        *.....12....*....
        *............45..
        * -> 123 * 12
        * * * * * * * * */
        int total = 0;
        Dictionary<Position, List<Number>> positionStarMap = new Dictionary<Position, List<Number>>();
        foreach (Number number in numbers) {           
            // find all the *
            int col_start = number.GetPosition().col - 1;
            int col_end = number.GetPosition().col + number.GetLength() + 1;
            int row = number.GetPosition().row;
            for (int cp = col_start; cp <= col_end; cp++) {
                if (cp >= 0 && cp < columns) {
                    // Check position above
                    if (row - 1 >= 0) {
                        char c_at_ps = map[row-1, cp];
                        if (c_at_ps == '*') {
                            total += calculate_gear_ratio(new Position(row-1, cp), number, positionStarMap);
                        }
                    }
                    // Check position below
                    if (row + 1 < rows) {
                        char c_at_ps = map[row+1, cp];
                        int row_px = row + 1;
                        if (c_at_ps == '*') {
                            total += calculate_gear_ratio(new Position(row+1, cp), number, positionStarMap);
                        }
                    }
                }
            }
            // Check left
            if (col_start >= 0) {
                char c_at_ps = map[row, col_start];
                if (c_at_ps == '*') {
                    total += calculate_gear_ratio(new Position(row, col_start), number, positionStarMap);
                }
            }
            // Check right
            if (col_end < columns) {
                char c_at_ps = map[row, col_end];
                if (c_at_ps == '*') {
                    total += calculate_gear_ratio(new Position(row, col_end), number, positionStarMap);
                }
            }
        }
        Console.WriteLine("Total: " + total);
    }

    static int calculate_gear_ratio(Position position, Number number, Dictionary<Position, List<Number>> positionStarMap) {
        // Check if star is already in the map
        if (positionStarMap.ContainsKey(position)) {
            int ration_to_add = 0;
            List<Number> numbers = positionStarMap[position];
            foreach (Number n in numbers) {
                ration_to_add += n.GetValue() * number.GetValue();
            }
            numbers.Add(number);
            return ration_to_add;
        } else {
            positionStarMap.Add(position, new List<Number> { number });
            return 0;
        }
    }

}



public struct Position {
    public int row;
    public int col;

    // Constructor
    public Position(int row, int col) {
        this.row = row;
        this.col = col;
    }
}


class Number {
    int? value;
    int? end_offset;
    Position position;
    List<char>  buffer;

    public Number(Position position) {
        this.position = position;
        buffer = new List<char>();
    }

    public void AddChar(char c) {
        if (char.IsDigit(c)) {
            buffer.Add(c);
        } else {
            throw new Exception("Invalid character for number");
        }
    }

    void Parse() {
        // Parse the buffer
        string s = new string(buffer.ToArray());
        value = int.Parse(s);
        end_offset = s.Length - 1;
    }

    public int GetValue() {
        if (value == null) {
            // Parse the buffer
            Parse();
        }
        return (int)value;
    }

    public int GetLength() {
        if (end_offset == null) {
            Parse();
        }
        return (int)end_offset;
    }

    public Position GetPosition() {
        return position;
    }
}