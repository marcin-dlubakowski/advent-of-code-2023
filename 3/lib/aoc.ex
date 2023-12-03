defmodule Aoc do
  use Application

  def get_numbers(filename) do
    File.stream!(filename)
    |> Stream.with_index()
    |> Stream.map(fn {line, i} ->
      Regex.scan(~r/\d+/, line, return: :index)
      |> List.flatten()
      |> Enum.map(fn {offset, length} ->
        {
          line |> String.slice(offset, length) |> String.to_integer(10),
          {i, offset},
          {i, offset + length - 1}
        }
      end)
    end)
    |> Enum.to_list()
    |> List.flatten()
  end

  def get_symbols(filename) do
    File.stream!(filename)
    |> Stream.with_index()
    |> Stream.map(fn {line, i} ->
      Regex.scan(~r/[^\d\.\n]/, line, return: :index)
      |> List.flatten()
      |> Enum.map(&{String.at(line, elem(&1, 0)), i, elem(&1, 0)})
    end)
    |> Enum.to_list()
    |> List.flatten()
  end

  def is_connected({_val, {minX, minY}, {_maxX, maxY}}, {_sym, symX, symY}) do
    minX in (symX - 1)..(symX + 1) &&
      (minY in (symY - 1)..(symY + 1) || maxY in (symY - 1)..(symY + 1))
  end

  def start(_type, _args) do
    part_one()
    part_two()
    {:ok, self()}
  end

  def part_one do
    numbers = get_numbers("input.txt")
    symbols = get_symbols("input.txt")

    result =
      numbers
      |> Enum.filter(fn number ->
        Enum.any?(symbols, fn symbol -> is_connected(number, symbol) end)
      end)
      |> Enum.map(&elem(&1, 0))
      |> Enum.sum()

    IO.puts("Result is: #{result}")
  end

  def part_two do
    numbers = get_numbers("input.txt")
    symbols = get_symbols("input.txt")

    result =
      symbols
      |> Enum.filter(&(elem(&1, 0) == "*"))
      |> Enum.map(fn gear ->
        {
          gear,
          numbers
          |> Enum.filter(fn number -> is_connected(number, gear) end)
          |> Enum.map(&elem(&1, 0))
        }
      end)
      |> Enum.filter(fn {_gear, numbers} -> length(numbers) == 2 end)
      |> Enum.map(fn {_gear, numbers} -> Enum.at(numbers, 0) * Enum.at(numbers, 1) end)
      |> Enum.sum()

    IO.puts("Result is: #{result}")
  end
end
