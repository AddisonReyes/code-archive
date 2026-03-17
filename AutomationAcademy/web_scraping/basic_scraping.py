import pprint
from typing import List

import requests
from bs4 import BeautifulSoup

# ----------------------------------------------------------
# ----------------- BeautifullSoup lib ---------------------
# ----------------------------------------------------------

# html_doc = """
# <html><head><title>The Dormouse's story</title></head>
# <body>
# <p class="title"><b>The Dormouse's story</b></p>

# <p class="story">Once upon a time there were three little sisters; and their names were
# <a href="http://example.com/elsie" class="sister" id="link1">Elsie</a>,
# <a href="http://example.com/lacie" class="sister" id="link2">Lacie</a> and
# <a href="http://example.com/tillie" class="sister" id="link3">Tillie</a>;
# and they lived at the bottom of a well.</p>

# <p class="story">...</p>
# """

# soup = BeautifulSoup(html_doc, "html.parser")

# print(soup.prettify())
# print(soup.title, "\n")
# print(soup.title.string)
# print(soup.p.b)

# print(soup.p["class"])
# print(soup.a["href"])

# print(soup.find(href="http://example.com/lacie"))
# print(soup.find(class_="story"))

# a_tags: List[str] = soup.find_all("a")
# pprint.pprint(a_tags)

# tags: List[str] = soup.find_all(["a", "title"])
# pprint.pprint(tags)

# p = soup.find(class_="story")
# pprint.pprint(p.contents)
# for child in p.children: print(child)

# body = soup.find("body")
# print(body.contents)
# print(len(body.contents))

# body = soup.find("body")
# body_tags = [tag for tag in body.contents if tag != "\n"]
# pprint.pprint(list(body_tags))
# print(len(list(body_tags)))

# p = soup.a.parent
# print(p)

# for p in soup.a.parents: print(p)

# a = soup.a
# print(a)
# print(a.next_sibling.next_sibling)
# print(a.next_sibling.next_sibling.next_sibling.next_sibling)


# ----------------------------------------------------------
# -------------------- Requests lib ------------------------
# ----------------------------------------------------------

url = "https://grokipedia.com/page/Web_scraping"
response = requests.get(url)

soup = BeautifulSoup(response.text, "html.parser")
# print(soup.prettify())

h1 = soup.find("h1")
print("h1: ", h1.text, "\n")

h2 = soup.find_all("h2")
print("h2 ->", len(h2))
for h in h2:
    print(h)
print("\n")

href = soup.a["href"]
print("href: ", href)
