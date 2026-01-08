from random import randint

from PIL import Image

image = Image.open("assets/raccoon.jpg")

# get one pixel
print(image.getpixel((0, 0)))

# greyscale images
# red_greyscale_image = image.getchannel("R")
# blue_greyscale_image = image.getchannel("B")

# red_greyscale_image.show()
# blue_greyscale_image.show()

for i in range(image.width):
    for j in range(image.height):
        r, g, b = image.getpixel((i, j))
        if i % 6 == 0 and j % 6 == 0:
            r = randint(0, 255)
            g = randint(0, 255)
            b = randint(0, 255)
            image.putpixel((i, j), (r, g, b))
image.show()
