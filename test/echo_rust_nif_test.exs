defmodule EchoRustNifTest do
  use ExUnit.Case
  doctest EchoRustNif

  test "greets the world" do
    assert EchoRustNif.hello() == :world
  end
end
