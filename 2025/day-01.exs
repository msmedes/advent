#!/usr/bin/env elixir

# Advent of Code 2025 - Day 1
# Run with: elixir day-01.exs

defmodule Day01 do
  def part1(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&parse_input/1)
    |> solve_part1()
  end

  def parse_input(line) do
    {char, amount} = String.split_at(line, 1)
    {char, String.to_integer(amount)}
  end

  def part2(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&parse_input/1)
    |> solve_part2()
  end

  defp solve_part1(rotations) do
    answer =
      rotations
      |> Enum.scan(50, fn {direction, amount}, acc ->
        case direction do
          "L" -> Integer.mod(acc - amount, 100)
          "R" -> Integer.mod(acc + amount, 100)
        end
      end)
      |> Enum.count(&(&1 == 0))

    answer
  end

  defp divmod(a, b) do
    {div(a, b), rem(a, b)}
  end

  defp solve_part2(rotations) do
    scanned =
      rotations
      |> Enum.scan({50, 0}, fn {direction, amount}, {acc, count} ->
        {full_rotations, steps} = divmod(amount, 100)

        new_position =
          case direction do
            "L" -> acc - steps
            "R" -> acc + steps
          end

        passed_or_landed = if acc != 0 and new_position not in 1..99, do: 1, else: 0

        {Integer.mod(new_position, 100), count + passed_or_landed + full_rotations}
      end)

    {_final_value, final_count} = List.last(scanned)

    final_count
  end
end

# Main execution
case System.argv() do
  [] ->
    input = File.read!("inputs/day01/input.txt")
    IO.puts("Part 1: #{Day01.part1(input)}")

    IO.puts("Part 2: #{Day01.part2(input)}")

  ["--test"] ->
    test_input = File.read!("inputs/day01/test_input.txt")
    IO.puts("Test Part 1: #{Day01.part1(test_input)}")

    IO.puts("Test Part 2: #{Day01.part2(test_input)}")

  _ ->
    IO.puts("Usage: elixir day-01.exs [--test]")
end
