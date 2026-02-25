from flask import render_template

from . import app
from .models import Recipe


@app.route("/")
def home():
    recipes = Recipe.query.all()
    return render_template("home.html", recipes=recipes)
