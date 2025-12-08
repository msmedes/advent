#!/usr/bin/env elixir

# Advent of Code 2025 - Day 4
# Run with: elixir day-04.exs

defmodule Day04 do
  @directions [{-1, -1}, {-1, 0}, {-1, 1}, {0, -1}, {0, 1}, {1, -1}, {1, 0}, {1, 1}]

  def part1(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> parse_grid()
    |> solve_part1()
  end

  def part2(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> parse_grid()
    |> solve_part2()
  end

  defp removable(grid, {row, col}, symbol) do
    count =
      @directions
      |> Enum.count(fn {dr, dc} ->
        Map.get(grid, {row + dr, col + dc}) == symbol
      end)

    count < 4
  end

  defp solve_part1(grid) do
    grid
    |> Map.filter(fn {_key, val} -> val == "@" end)
    |> Enum.map(fn {k, _} -> removable(grid, k, "@") end)
    # |> IO.inspect(charlist: :aslist)
    |> Enum.count(fn val -> val end)
  end

  defp remove_pass(grid) do
    new_grid =
      Map.new(grid, fn {coord, value} ->
        if value == "@" and removable(grid, coord, "@") do
          {coord, "."}
        else
          {coord, value}
        end
      end)

    removed_count =
      Enum.count(grid, fn {coord, value} ->
        value == "@" and Map.get(new_grid, coord) == "."
      end)

    {new_grid, removed_count}
  end

  defp remove_all(grid, total_removed \\ 0) do
    {new_grid, removed_this_pass} = remove_pass(grid)

    if removed_this_pass == 0 do
      {grid, total_removed}
    else
      remove_all(new_grid, total_removed + removed_this_pass)
    end
  end

  defp solve_part2(grid) do
    {_grid, total_removed} = remove_all(grid)
    total_removed
  end

  defp parse_grid(lines) do
    for {line, row} <- Enum.with_index(lines),
        {char, col} <- Enum.with_index(String.graphemes(line)),
        into: %{} do
      {{row, col}, char}
    end
  end
end

# Main execution
case System.argv() do
  [] ->
    input = File.read!("inputs/day04/input.txt")
    IO.puts("Part 1: #{Day04.part1(input)}")
    IO.puts("Part 2: #{Day04.part2(input)}")

  ["--test"] ->
    test_input = File.read!("inputs/day04/test_input.txt")
    IO.puts("Test Part 1: #{Day04.part1(test_input)}")
    IO.puts("Test Part 2: #{Day04.part2(test_input)}")

  _ ->
    IO.puts("Usage: elixir day-04.exs [--test]")
end
