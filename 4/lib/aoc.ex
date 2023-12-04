defmodule Aoc do
  use Application

  def start(_type, _args) do
    part_one()
    part_two()
    {:ok, self()}
  end

  def decode_cards do
    File.stream!("input.txt")
    |> Stream.map(&String.replace(String.trim_trailing(&1), ~r/ +/, " "))
    |> Stream.map(&String.split(&1, ": "))
    |> Stream.map(fn data -> String.split(Enum.at(data, 1), " | ") end)
    |> Stream.map(fn data ->
      {
        Enum.at(data, 0) |> String.split(" ") |> Enum.map(&String.to_integer(&1, 10)),
        Enum.at(data, 1) |> String.split(" ") |> Enum.map(&String.to_integer(&1, 10))
      }
    end)
    |> Enum.to_list()
  end

  def points(x) when x > 0, do: 2 ** (x - 1)
  def points(x) when x <= 0, do: 0

  def part_one do
    cards = decode_cards()

    result =
      cards
      |> Enum.map(fn {winners, numbers} ->
        numbers |> Enum.filter(&Enum.member?(winners, &1)) |> length() |> points()
      end)
      |> Enum.sum()

    IO.inspect("Result is: #{result}")
  end

  def part_two do
  end
end
