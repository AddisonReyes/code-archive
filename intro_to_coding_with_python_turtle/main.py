from turtle import *


# create a square
def square(pixels: int = 50):
    for _ in range(4):
        forward(pixels)
        right(90)


def create_circle(radius: float = 50, ccolor: str = ""):
    color(ccolor)
    begin_fill()
    circle(radius)
    end_fill()


# create a triangle
def triangle(pixels: float = 50):
    for _ in range(3):
        forward(pixels)
        left(120)


def flower():
    for i in range(4):
        create_circle(60, "pink")
        create_circle(40, "violet")
        create_circle(20, "purple")
        right(90)


if __name__ == "__main__":
    triangle(100)

    done()
