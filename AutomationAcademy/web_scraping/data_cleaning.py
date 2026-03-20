import pprint
import re

import requests
from bs4 import BeautifulSoup

URL: str = "https://es.wikipedia.org/wiki/Undertale"
PATH: str = "./html/undertale.html"


def get_html(url: str, path: str) -> None:
    response = requests.get(
        url=url,
        headers={"User-Agent": "Dako"},
    )

    with open(path, "w", encoding="utf-8") as f:
        f.write(response.text)


def main() -> None:
    # get_html(URL, PATH)

    with open(PATH, "r", encoding="utf-8") as f:
        html = f.read()

    soup = BeautifulSoup(html, "html.parser")

    section_headings = soup.find_all("h2")
    section_headings = [span.string for span in section_headings]
    print(section_headings)
    print(type(section_headings[0]))


if __name__ == "__main__":
    main()
