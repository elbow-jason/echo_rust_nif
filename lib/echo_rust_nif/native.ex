defmodule EchoRustNif.Native do
  use Rustler, otp_app: :echo_rust_nif, crate: "echorustnif_native"

  defp err, do: :erlang.nif_error(:nif_not_loaded)

  def echoer_start, do: err()
  def echoer_stop(_ref), do: err()
  def echoer_echo(_ref, _message), do: err()
end
