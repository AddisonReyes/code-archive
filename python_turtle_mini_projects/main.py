from random import *
from turtle import *


def move_and_turn(distance, angle):
    forward(distance)
    right(angle)


def plus(px: int = 100, c: str = "red"):
    color(c)
    begin_fill()
    move_and_turn(px, 90)
    move_and_turn(px, -90)
    move_and_turn(px, 90)
    move_and_turn(px, -90)
    move_and_turn(-px, 90)
    move_and_turn(px, -90)
    move_and_turn(-px, 90)
    move_and_turn(-px, 90)
    move_and_turn(px, 90)
    move_and_turn(px, -90)
    move_and_turn(-px, 90)
    move_and_turn(px, 90)
    end_fill()


def random_shi():
    _min = 10
    _max = 100
    for _ in range(6000):
        f = randrange(_min, _max)
        a = randrange(_min, _max)
        move_and_turn(f, a)


if __name__ == "__main__":
    for i in range(1000):
        if i % 2 == 0:
            move_and_turn(i * 0.6, 6)
        else:
            move_and_turn(6, i * 0.6)
        write(i)
    done()
