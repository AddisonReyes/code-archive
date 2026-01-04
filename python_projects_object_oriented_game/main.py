from typing import List


class GameObject:
    def __init__(
        self,
        name: str = "",
        appearance: str = "",
        feel: str = "",
        smell: str = "",
    ):
        self.name = name
        self.appearance = appearance
        self.feel = feel
        self.smell = smell

    def look(self):
        return f"You look at the {self.name}. {self.appearance}\n"

    def touch(self):
        return f"You touch the {self.name}. {self.feel}\n"

    def sniff(self):
        return f"You sniff the {self.name}. {self.smell}\n"


class Room:
    def __init__(
        self,
        escape_code: int = 0,
        game_objects: List[GameObject] = [],
    ):
        self.escape_code = escape_code
        self.game_objects = game_objects

    def check_code(self, code: int):
        return code == self.escape_code

    def get_game_objects_names(self):
        return [game_object.name for game_object in self.game_objects]


class Game:
    def __init__(self):
        self.attempts = 0
        self.objects = self.create_game_objects()
        self.room = Room(111, [])

    def create_game_objects(self):
        return [
            GameObject(
                "Knife",
                "Some appearance",
                "Some feel",
                "Some smell",
            ),
            GameObject(
                "Knife",
                "Some appearance",
                "Some feel",
                "Some smell",
            ),
        ]


def main():
    game_object = GameObject(
        "Knife",
        "Some appearance",
        "Some feel",
        "Some smell",
    )

    print(game_object.look())
    print(game_object.touch())
    print(game_object.sniff())


if __name__ == "__main__":
    main()
