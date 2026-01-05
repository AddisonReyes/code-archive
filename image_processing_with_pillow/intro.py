from PIL import Image


def main():
    img_path = "assets/cat.jpg"

    image = Image.open(img_path)
    print(image.filename)
    print(image.size)
    print(image.format)

    # image = image.transpose(Image.FLIP_LEFT_RIGHT)
    image = image.rotate(30)

    image.save("assets/cat_rotated.png", "png")
    # image.show()


if __name__ == "__main__":
    main()
