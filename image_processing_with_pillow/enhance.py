from PIL import Image, ImageEnhance

img = Image.open("./assets/cat.jpg")

enhancer = ImageEnhance.Color(img)
enhancer.enhance(2.6).show()

enhancer = ImageEnhance.Contrast(img)
enhancer.enhance(2.6).show()

enhancer = ImageEnhance.Brightness(img)
enhancer.enhance(2.6).show()

enhancer = ImageEnhance.Sharpness(img)
enhancer.enhance(2.6).show()

img.show()
