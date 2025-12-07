# Advent of Code 2025 - Elixir Solutions

## Structure

Each day has its own script file: `day-XX.exs` where XX is the day number (01-25).

## Creating a New Day

Use the helper script to create a new day template:

```bash
elixir new_day.exs 1
```

This will create `day-01.exs` with a basic template.

## Running a Day

Run a day's script with:

```bash
elixir day-01.exs
```

This will read from `input.txt` in the same directory.

To run with test input:

```bash
elixir day-01.exs --test
```

This will read from `test_input.txt` instead.

```bash
# Create day 1
elixir new_day.exs 1

# Add input
# (paste input into input.txt)

# Run it
elixir day-01.exs
```

