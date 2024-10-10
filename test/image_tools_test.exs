defmodule ImageToolsTest do
  use ExUnit.Case
  import ExUnit.CaptureIO
  @test_path Path.join(__DIR__, "assets/sample.jpg")
  @test_image File.read!(@test_path)

  test "rotate/1 rotates an image" do
    assert {:ok, @test_path} = ImageTools.rotate(@test_path)
  end

  test "resize/2 resizes an image" do
    assert {:ok, @test_path} = ImageTools.resize(@test_path, 5000)
    assert {:ok, @test_path} = ImageTools.resize(@test_path, 3456)
  end

  test "create_thumbnail/4 creates a thumbnail successfully" do
    assert {:ok, binary} = ImageTools.create_thumbnail(@test_image, 320, 240)
    assert is_binary(binary)
  end

  test "create_thumbnail/4 handles quality option" do
    assert {:ok, binary} = ImageTools.create_thumbnail(@test_image, 320, 240, quality: 50)
    assert is_binary(binary)
  end

  test "create_thumbnail/4 handles target_size option" do
    assert {:ok, binary} = ImageTools.create_thumbnail(@test_image, 320, 240, target_size: 12_000)
    assert is_binary(binary)
  end

  test "create_thumbnail/4 handles invalid input" do
    assert {:error, "width must be > 0"} = ImageTools.create_thumbnail(@test_image, 0, 240)
    assert {:error, "height must be > 0"} = ImageTools.create_thumbnail(@test_image, 320, 0)

    assert {:error, "quality must be >= 0"} =
             ImageTools.create_thumbnail(@test_image, 320, 240, quality: -1)

    assert {:error, "quality must be <= 100"} =
             ImageTools.create_thumbnail(@test_image, 320, 240, quality: 101)

    assert {:error, "target_size must be > 0"} =
             ImageTools.create_thumbnail(@test_image, 320, 240, target_size: 0)
  end
end
