defmodule EchoRustNif.MixProject do
  use Mix.Project

  def project do
    [
      app: :echo_rust_nif,
      version: "0.1.0",
      elixir: "~> 1.11",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      rustler_crates: rustler_crates(),
      compilers: [:rustler] ++ Mix.compilers()
    ]
  end

  defp rustler_crates do
    [echo_rust_nif: [
      path: "native/echorustnif_native",
      mode: rustc_mode(Mix.env)
    ]]
  end


  defp rustc_mode(:prod), do: :release
  defp rustc_mode(_), do: :debug


  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.22.0-rc.0"},
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
    ]
  end
end
