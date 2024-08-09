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

  @spec create_thumbnail(binary(), non_neg_integer(), non_neg_integer()) ::
          {:ok, binary()} | {:error, String.t()}
  def create_thumbnail(body, width, height) do
    create_thumbnail(body, width, height, nil, nil)
  end

  @doc """
  Create a thumbnail image with options

  # Quality

  The quality ranges from 0 to 100, defaulting to 75.

  # Target size

  You can also specify a target size in bytes, although this will increase the processing time
  by approximately 20% to 80%.

  ## Examples

      iex> content = File.read!("./test/assets/images/sample.jpg")
      iex> ImageTools.create_thumbnail(content, 320, 240, quality: 50)
      iex> ImageTools.create_thumbnail(content, 320, 240, target_size: 12_000)
  """
  @spec create_thumbnail(binary(), non_neg_integer(), non_neg_integer(), quality: float()) ::
          {:ok, binary()} | {:error, String.t()}
  def create_thumbnail(body, width, height, quality: quality) when is_number(quality),
    do: create_thumbnail(body, width, height, quality / 1, nil)

  @spec create_thumbnail(binary(), non_neg_integer(), non_neg_integer(), target_size: non_neg_integer()) ::
          {:ok, binary()} | {:error, String.t()}
  def create_thumbnail(body, width, height, target_size: target_size) when is_integer(target_size),
    do: create_thumbnail(body, width, height, nil, target_size)

  defp create_thumbnail(body, _, _, _, _) when is_nil(body), do: {:error, "body is empty"}
  defp create_thumbnail(_, width, _, _, _) when width <= 0, do: {:error, "width must be > 0"}
  defp create_thumbnail(_, _, height, _, _) when height <= 0, do: {:error, "height must be > 0"}

  defp create_thumbnail(_, _, _, quality, _) when not is_nil(quality) and quality < 0,
    do: {:error, "quality must be >= 0"}

  defp create_thumbnail(_, _, _, quality, _) when not is_nil(quality) and quality > 100,
    do: {:error, "quality must be <= 100"}

  defp create_thumbnail(_, _, _, _, target_size) when not is_nil(target_size) and target_size <= 0,
    do: {:error, "target_size must be > 0"}

  @spec create_thumbnail(binary(), non_neg_integer(), non_neg_integer(), float() | nil, non_neg_integer() | nil) ::
          {:ok, binary()} | {:error, String.t()}
  defp create_thumbnail(body, width, height, quality, target_size) do
    _create_thumbnail(body, width, height, quality, target_size)
  end

  # NIF function definition
  @spec _create_thumbnail(binary(), non_neg_integer(), non_neg_integer(), float() | nil, non_neg_integer() | nil) ::
          {:ok, binary()} | {:error, String.t()}
  defp _create_thumbnail(_body, _width, _height, _quality, _target_size) do
    :erlang.nif_error(:nif_not_loaded)
  end
end
