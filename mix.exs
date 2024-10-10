defmodule ImageTools.MixProject do
  use Mix.Project
  @version "0.1.17"

  def project do
    [
      app: :image_tools,
      description: "A set of simple image tools like rotate an image implemented in Rust",
      package: package(),
      version: @version,
      elixir: "~> 1.17",
      build_embedded: Mix.env() == :prod,
      deps: deps(),
      aliases: aliases()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.34.0"}
    ]
  end

  defp package() do
    [
      name: "image_tools",
      licenses: ["Unlicense"],
      links: %{"GitHub" => "https://github.com/matthiaz/image_tools"},
      files: ~w(mix.exs README.md lib native test .formatter.exs priv),
      exclude_patterns: ~w(target _build)
    ]
  end

  defp aliases do
    if Mix.env() == :prod do
      []
    else
      [
        compile: ["precompile", "compile"],
        release: ["precompile", "release"],
      ]
    end
  end
end
