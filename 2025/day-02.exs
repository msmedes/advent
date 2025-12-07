defmodule Day02 do
  def part1(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&parse_input/1)
    |> solve_part1()
  end

  def part2(input) do
    input
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&parse_input/1)
    |> solve_part2()
  end

  defp parse_input(line) do
    String.split(line, ",")
    |> Enum.map(&String.split(&1, "-"))
    |> Enum.map(fn range -> Enum.map(range, &String.to_integer/1) end)
  end

  defp is_invalid_code(code) do
    digits = Integer.digits(code)

    if rem(length(digits), 2) != 0 do
      false
    else
      midpoint = div(length(digits), 2)
      {first_half, second_half} = Enum.split(digits, midpoint)
      first_half == second_half
    end
  end

  defp solve_part1(id_ranges) do
    result =
      id_ranges
      |> Enum.flat_map(& &1)
      |> Enum.map(fn [start, finish] ->
        start..finish
        |> Enum.filter(&is_invalid_code/1)
        |> Enum.sum()
      end)
      |> Enum.sum()

    result
  end

  defp is_invalid_code2(id_range) do
    digits = Integer.digits(id_range)
    len = length(digits)

    if len < 2 do
      false
    else
      1..div(len, 2)
      |> Enum.any?(fn pattern_len ->
        chunks = Enum.chunk_every(digits, pattern_len)
        length(chunks) >= 2 and Enum.uniq(chunks) == [hd(chunks)]
      end)
    end
  end

  defp solve_part2(id_ranges) do
    result =
      id_ranges
      |> Enum.flat_map(& &1)
      |> Enum.map(fn [start, finish] ->
        start..finish
        |> Enum.filter(&is_invalid_code2/1)
        |> Enum.sum()
      end)
      |> Enum.sum()

    result
  end
end

case System.argv() do
  [] ->
    input = File.read!("inputs/day02/input.txt")
    IO.puts("Part 1: #{Day02.part1(input)}")
    IO.puts("Part 2: #{Day02.part2(input)}")

  ["--test"] ->
    test_input = File.read!("inputs/day02/test_input.txt")
    IO.puts("Test Part 1: #{Day02.part1(test_input)}")
    IO.puts("Test Part 2: #{Day02.part2(test_input)}")

  _ ->
    IO.puts("Usage: elixir day-02.exs [--test]")
end
