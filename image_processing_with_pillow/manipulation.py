from PIL import Image


def main():
    img_path = "assets/cat.jpg"

    image = Image.open(img_path)

    image_transposed = image.transpose(Image.FLIP_LEFT_RIGHT)

    image_rotated = image.rotate(45, expand=True)

    image_crop = image.crop((800, 600, 1600, 1600))

    image_resize = image.resize((1000, 600))

    combined_image = (
        image.transpose(Image.FLIP_LEFT_RIGHT)
        .rotate(45, expand=True)
        .crop((800, 600, 1600, 1600))
        .resize((1000, 600))
    )
    combined_image.show()

    # image_resize.show()


if __name__ == "__main__":
    main()
