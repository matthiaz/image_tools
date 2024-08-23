defmodule Mix.Tasks.Precompile do
  use Mix.Task

  @shortdoc "Checks and installs Rust and Cargo before compilation"

  def run(_args) do
    unless system_has_rust?() do
      Mix.shell().info("Rust and Cargo not found. Installing...")
      install_rust()
    end
  end

  defp system_has_rust? do
    System.find_executable("rustc") && System.find_executable("cargo")
  end

  defp install_rust do
    case :os.type() do
      {:win32, _} ->
        Mix.raise("Please install Rust and Cargo manually on Windows: https://rustup.rs")

      _ ->
        Mix.shell().cmd("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y", into: IO.stream(:stdio, :line))
        Mix.shell().cmd("source $HOME/.cargo/env", into: IO.stream(:stdio, :line))
    end
  end
end
