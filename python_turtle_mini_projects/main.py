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


if __name__ == "__main__":
    plus()
    done()
