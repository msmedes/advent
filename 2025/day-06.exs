#!/usr/bin/env elixir

# Advent of Code 2025 - Day 6
# Run with: elixir day-06.exs

defmodule Day06 do
  def part1(input) do
    input
    |> String.trim()
    |> parse_input
    |> solve_part1()
  end

  def part2(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> solve_part2()
  end

  defp parse_input(lines) do
    lines
    |> String.split("\n", trim: true)
    |> Enum.map(&String.split(&1))
    |> Enum.zip_with(& &1)
  end

  defp solve_part1(problems) do
    problems
    |> Enum.map(fn problem ->
      {numbers, [operator]} = Enum.split(problem, -1)
      numbers = Enum.map(numbers, &String.to_integer(&1))

      case operator do
        "*" ->
          Enum.reduce(numbers, &*/2)

        "+" ->
          Enum.reduce(numbers, &+/2)
      end
    end)
    |> Enum.sum()
  end

  defp solve_part2(lines) do
    # TODO: Implement part 2
    lines
  end
end

# Main execution
case System.argv() do
  [] ->
    input = File.read!("inputs/day06/input.txt")
    IO.puts("Part 1: #{Day06.part1(input)}")

  # IO.puts("Part 2: #{Day06.part2(input)}")

  ["--test"] ->
    test_input = File.read!("inputs/day06/test_input.txt")
    IO.puts("Test Part 1: #{Day06.part1(test_input)}")

  # IO.puts("Test Part 2: #{Day06.part2(test_input)}")

  _ ->
    IO.puts("Usage: elixir day-06.exs [--test]")
end
