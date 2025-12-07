#!/usr/bin/env elixir

# Helper script to create a new day template
# Usage: elixir new_day.exs <day_number>

[day_num | _] = System.argv()

if day_num == nil do
  IO.puts("Usage: elixir new_day.exs <day_number>")
  System.halt(1)
end

day_num = String.pad_leading(day_num, 2, "0")
filename = "day-#{day_num}.exs"

if File.exists?(filename) do
  IO.puts("File #{filename} already exists!")
  System.halt(1)
end

template = """
#!/usr/bin/env elixir

# Advent of Code 2025 - Day #{String.to_integer(day_num)}
# Run with: elixir #{filename}

defmodule Day#{day_num} do
  def part1(input) do
    input
    |> String.trim()
    |> String.split("\\n")
    |> solve_part1()
  end

  def part2(input) do
    input
    |> String.trim()
    |> String.split("\\n")
    |> solve_part2()
  end

  defp solve_part1(lines) do
    # TODO: Implement part 1
    lines
  end

  defp solve_part2(lines) do
    # TODO: Implement part 2
    lines
  end
end

# Main execution
case System.argv() do
  [] ->
    input = File.read!("inputs/day#{day_num}/input.txt")
    IO.puts("Part 1: #{Day#{day_num}.part1(input)}")
    IO.puts("Part 2: #{Day#{day_num}.part2(input)}")

  ["--test"] ->
    test_input = File.read!("inputs/day#{day_num}/test_input.txt")
    IO.puts("Test Part 1: #{Day#{day_num}.part1(test_input)}")
    IO.puts("Test Part 2: #{Day#{day_num}.part2(test_input)}")

  _ ->
    IO.puts("Usage: elixir #{filename} [--test]")
end
"""

File.write!(filename, template)
IO.puts("Created #{filename}")
IO.puts("Don't forget to create input.txt and test_input.txt files!")
