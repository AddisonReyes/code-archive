from turtle import *

diameter: int = 40
pop_diameter: int = 100


def draw_balloon():
    color("red")
    dot(diameter)


def inflate_balloon(increase: int = 10):
    global diameter
    diameter += increase
    draw_balloon()

    if diameter >= pop_diameter:
        clear()
        diameter = 40
        write("POP!")


def main():
    draw_balloon()

    onkey(inflate_balloon, "space")
    listen()

    done()


if __name__ == "__main__":
    main()
