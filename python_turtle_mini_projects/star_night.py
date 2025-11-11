from random import *
from turtle import *


def star(x: int, y: int):
    penup()
    goto(x, y)
    pendown()

    diameter = randrange(1, 10)
    color("white")
    dot(diameter)


def main():
    bgcolor("black")
    hideturtle()
    stars = 0

    speed(0)

    while stars < 1000:
        width: int = window_width() // 2
        height: int = window_height() // 2

        x: int = randrange(-width, width)
        y: int = randrange(-height, height)
        star(x, y)

        stars += 1

    write("pluhh")

    done()


if __name__ == "__main__":
    main()
