from turtle import *


# create a square
def square(square_size: int = 100):
    for _ in range(4):
        forward(square_size)
        right(90)


def create_circle(radius: float = 50):
    circle(radius)


if __name__ == "__main__":
    create_circle(90)
    create_circle(200)

    done()
