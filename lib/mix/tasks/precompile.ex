defmodule Mix.Tasks.Precompile do
  use Mix.Task

  @shortdoc "Checks and installs Rust and Cargo before compilation"

  def run(_args) do
    Mix.shell().info("Checking if rust and cargo are already installed...")
    if system_has_rust?() do
      Mix.shell().info("Yep, all good.")
    else
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
        Mix.shell().info("Downloading for Linux...")
        Mix.shell().cmd("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal", into: IO.stream(:stdio, :line))
        Mix.shell().cmd(". $HOME/.cargo/env", into: IO.stream(:stdio, :line))
        Mix.shell().cmd("touch $HOME/.bashrc && echo 'source $HOME/.cargo/env' >> $HOME/.bashrc", into: IO.stream(:stdio, :line))
    end
  end
end
