using System;
using System.Collections.Generic;
using System.IO;

class Program
{
    static void Main()
    {
        string[] map = ReadMapFromFile("input/input1.txt");

        if (map.Length == 0)
        {
            Console.WriteLine("The map file is empty or invalid.");
            return;
        }

        int result = CalculateUniqueAntinodes(map);
        Console.WriteLine($"Unique antinode locations: {result}");
    }

    static string[] ReadMapFromFile(string filePath)
    {
        try
        {
            return File.ReadAllLines(filePath);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error reading file: {ex.Message}");
            return Array.Empty<string>();
        }
    }

    static int CalculateUniqueAntinodes(string[] map)
    {
        var antennas = ParseAntennas(map);
        var antinodes = FindAntinodes(map, antennas);
        return antinodes.Count;
    }

    static Dictionary<char, List<(int row, int col)>> ParseAntennas(string[] map)
    {
        var antennas = new Dictionary<char, List<(int, int)>>();

        for (int row = 0; row < map.Length; row++)
        {
            for (int col = 0; col < map[row].Length; col++)
            {
                char cell = map[row][col];
                if (char.IsLetterOrDigit(cell))
                {
                    if (!antennas.ContainsKey(cell))
                        antennas[cell] = new List<(int, int)>();
                    antennas[cell].Add((row, col));
                }
            }
        }

        return antennas;
    }

    static HashSet<(int, int)> FindAntinodes(string[] map, Dictionary<char, List<(int row, int col)>> antennas)
    {
        var antinodes = new HashSet<(int, int)>();
        int rows = map.Length;
        int cols = map[0].Length;

        foreach (var antennaGroup in antennas)
        {
            var positions = antennaGroup.Value;

            for (int i = 0; i < positions.Count; i++)
            {
                for (int j = i + 1; j < positions.Count; j++)
                {
                    AddAntinodesForPair(positions[i], positions[j], antinodes, rows, cols);
                }
            }
        }

        return antinodes;
    }

    static void AddAntinodesForPair((int row1, int col1) pos1, (int row2, int col2) pos2, HashSet<(int, int)> antinodes, int rows, int cols)
    {
        int dRow = pos2.row2 - pos1.row1;
        int dCol = pos2.col2 - pos1.col1;

        // Calculate potential antinodes
        var antinode1 = (pos1.row1 - dRow, pos1.col1 - dCol);
        var antinode2 = (pos2.row2 + dRow, pos2.col2 + dCol);

        // Add to the set if within bounds
        if (IsWithinBounds(antinodes, rows, cols, antinode1))
            antinodes.Add(antinode1);

        if (IsWithinBounds(antinodes, rows, cols, antinode2))
            antinodes.Add(antinode2);
    }

    static bool IsWithinBounds(HashSet<(int, int)> antinodes, int rows, int cols, (int row, int col) position)
    {
        return position.row >= 0 && position.row < rows &&
               position.col >= 0 && position.col < cols;
    }
}
