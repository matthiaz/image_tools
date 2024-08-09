defmodule ImageTools do
  use Rustler, otp_app: :image_tools, crate: "image_tools"
  @moduledoc """
  Documentation for `ImageTools`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> ImageTools.hello()
      :world

  """
  def hello do
    :world
  end
  def rotate_image(_binary, _degrees), do: :erlang.nif_error(:nif_not_loaded)
end
