defmodule Aoc do
  use Application

  def get_numbers(filename) do
    File.stream!(filename)
    |> Stream.with_index()
    |> Stream.map(fn {line, i} ->
      Regex.scan(~r/\d+/, line, return: :index)
      |> List.flatten()
      |> Enum.map(fn t ->
        {
          line |> String.slice(elem(t, 0), elem(t, 1)) |> String.to_integer(10),
          {i, elem(t, 0)},
          {i, elem(t, 0) + elem(t, 1) - 1}
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
      |> Enum.map(&{i, elem(&1, 0)})
    end)
    |> Enum.to_list()
    |> List.flatten()
  end

  def start(_type, _args) do
    numbers = get_numbers("input.txt")
    symbols = get_symbols("input.txt")

    IO.inspect(numbers)
    IO.inspect(symbols)

    result =
      numbers
      |> Enum.filter(fn number ->
        symbols
        |> Enum.any?(fn symbol ->
          {x, minY} = elem(number, 1)
          {_, maxY} = elem(number, 2)
          {resX, resY} = symbol

          x in (resX - 1)..(resX + 1) &&
            (minY in (resY - 1)..(resY + 1) || maxY in (resY - 1)..(resY + 1))
        end)
      end)
      |> Enum.map(&elem(&1, 0))
      |> Enum.sum()

    IO.puts("Result is: #{result}")

    {:ok, self()}
  end
end
