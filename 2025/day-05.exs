#!/usr/bin/env elixir

# Advent of Code 2025 - Day 5
# Run with: elixir day-05.exs

defmodule Day05 do
  def part1(input) do
    input
    |> String.trim()
    |> parse_input()
    |> solve_part1()
  end

  def part2(input) do
    input
    |> String.trim()
    |> parse_input()
    |> solve_part2()
  end

  defp solve_part1({fresh_ranges, ingredients}) do
    ingredients
    |> Enum.count(fn ingredient ->
      Enum.any?(fresh_ranges, fn fresh_range ->
        ingredient in fresh_range
      end)
    end)
  end

  defp merge_ranges([range]), do: [range]

  defp merge_ranges([range1, range2 | rest]) do
    case Range.disjoint?(range1, range2) do
      true ->
        [range1 | merge_ranges([range2 | rest])]

      false ->
        merged_range = min(range1.first, range2.first)..max(range1.last, range2.last)
        merge_ranges([merged_range | rest])
    end
  end

  defp solve_part2({fresh_ranges, _ingredients}) do
    fresh_ranges |> Enum.sort() |> merge_ranges |> Enum.reduce(0, &(Range.size(&1) + &2))
  end

  defp parse_input(input) do
    [fresh_ranges, ingredients] = String.split(input, "\n\n")

    fresh_ranges =
      fresh_ranges
      |> String.split("\n", trim: true)
      |> Enum.map(fn range ->
        [from, to] = String.split(range, "-") |> Enum.map(&String.to_integer/1)
        from..to
      end)

    ingredients = ingredients |> String.split("\n", trim: true) |> Enum.map(&String.to_integer/1)

    {fresh_ranges, ingredients}
  end
end

# Main execution
case System.argv() do
  [] ->
    input = File.read!("inputs/day05/input.txt")
    IO.puts("Part 1: #{Day05.part1(input)}")

    IO.puts("Part 2: #{Day05.part2(input)}")

  ["--test"] ->
    test_input = File.read!("inputs/day05/test_input.txt")
    IO.puts("Test Part 1: #{Day05.part1(test_input)}")

    IO.puts("Test Part 2: #{Day05.part2(test_input)}")

  _ ->
    IO.puts("Usage: elixir day-05.exs [--test]")
end
