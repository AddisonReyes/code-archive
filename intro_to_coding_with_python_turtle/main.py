from turtle import *


# create a square
def square(square_size: int = 100):
    for _ in range(4):
        forward(square_size)
        right(90)


def create_circle(radius: float = 50, ccolor: str = ""):
    color(ccolor)
    begin_fill()
    circle(radius)
    end_fill()


if __name__ == "__main__":
    for i in range(4):
        create_circle(60, "pink")
        create_circle(40, "violet")
        create_circle(20, "purple")

        right(90)

    done()
