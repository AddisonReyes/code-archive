from turtle import (
    begin_fill,
    circle,
    color,
    done,
    end_fill,
    forward,
    left,
    pendown,
    penup,
    right,
)


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


def house(walls: int = 50, roof: int = 70):
    color("yellow")
    begin_fill()
    square(walls)
    end_fill()

    penup()
    diff = abs(walls - roof) / 2
    if roof > walls:
        forward(-diff)
    elif walls > roof:
        forward(diff)
    pendown()

    color("red")
    begin_fill()
    triangle(roof)
    end_fill()


if __name__ == "__main__":
    # triangle(100)

    # circle(50)
    # penup()
    # forward(150)
    # pendown()
    # circle(50)

    house(walls=100, roof=170)

    done()
