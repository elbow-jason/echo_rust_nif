defmodule EchoRustNif do

  @moduledoc """
  Documentation for `EchoRustNif`.
  """
  alias EchoRustNif.Native

  def start, do: Native.echoer_start()

  def stop(ref) when is_reference(ref), do: Native.echoer_stop(ref)

  def echo(ref, msg) when is_reference(ref), do: Native.echoer_echo(ref, msg)
end
