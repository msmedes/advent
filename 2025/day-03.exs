defmodule Day03 do
  def part1(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_input/1)
    |> solve_part1()
  end

  def part2(input) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_input/1)
    |> solve_part2()
  end

  defp parse_input(line) do
    line |> String.to_integer() |> Integer.digits()
  end

  defp max_joltage(bank) do
    {_max_seen, best_pair} =
      bank
      |> Enum.reduce({0, 0}, fn battery, {max_seen, best_pair} ->
        candidate = max(max_seen * 10 + battery, best_pair)
        max_seen = max(max_seen, battery)
        {max_seen, candidate}
      end)

    best_pair
  end

  defp max_joltage2(bank) do
    {_remaining_batteries, result} =
      12..1
      |> Enum.reduce({bank, []}, fn remaining_to_pick, {bank, result} ->
        search_limit = length(bank) - remaining_to_pick + 1
        searchable = Enum.take(bank, search_limit)

        {max_val, max_idx} =
          searchable |> Enum.with_index() |> Enum.max_by(fn {val, _idx} -> val end)

        remaining_batteries = Enum.drop(bank, max_idx + 1)
        new_result = result ++ [max_val]
        {remaining_batteries, new_result}
      end)

    Integer.undigits(result)
  end

  defp solve_part1(lines) do
    lines
    |> Enum.map(&max_joltage/1)
    # |> IO.inspect(charlists: :as_lists)
    |> Enum.sum()
  end

  defp solve_part2(lines) do
    lines
    |> Enum.map(&max_joltage2/1)
    |> Enum.sum()
  end
end

# Main execution
case System.argv() do
  [] ->
    input = File.read!("inputs/day03/input.txt")
    IO.puts("Part 1: #{Day03.part1(input)}")
    IO.puts("Part 2: #{Day03.part2(input)}")

  ["--test"] ->
    test_input = File.read!("inputs/day03/test_input.txt")
    IO.puts("Test Part 1: #{Day03.part1(test_input)}")
    IO.puts("Test Part 2: #{Day03.part2(test_input)}")

  _ ->
    IO.puts("Usage: elixir day-03.exs [--test]")
end
