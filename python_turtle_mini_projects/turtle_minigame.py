from turtle import *


class Maria:
    def __init__(self, x: int = -200, y: int = 0):
        self.speed: float = 32.0
        self.can_move = True
        self.x: int = x
        self.y: int = y

        color("green")
        shape("turtle")

        onkey(self.up, "w")
        onkey(self.left, "a")
        onkey(self.down, "s")
        onkey(self.right, "d")
        listen()

    def up(self):
        if self.can_move:
            self.y += self.speed
            setheading(90)

    def down(self):
        if self.can_move:
            self.y -= self.speed
            setheading(-90)

    def right(self):
        if self.can_move:
            self.x += self.speed
            setheading(0)

    def left(self):
        if self.can_move:
            self.x -= self.speed
            setheading(-180)

    def update(self):
        if self.can_move:
            goto(self.x, self.y)


def ocean():
    color("blue")

    penup()
    goto(200, 450)
    pendown()

    begin_fill()
    goto(500, 450)
    goto(500, -450)
    goto(200, -450)
    goto(200, 450)
    end_fill()
    penup()


def main():
    bgcolor("orange")
    speed(0)
    ocean()

    maria = Maria()

    goal = 240
    while maria.x < goal:
        maria.update()

    color("white")
    write("pluhh")
    goto(9999, 9999)

    done()


if __name__ == "__main__":
    main()
