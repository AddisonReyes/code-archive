from flask import redirect, render_template, url_for
from flask_login import current_user

from . import app, db
from .forms import RecipeForm
from .models import Recipe, RegistrationForm, User


@app.route("/")
def home():
    recipes = Recipe.query.all()
    return render_template("home.html", recipes=recipes)


@app.route("/new_recipe", methods=["GET", "POST"])
def new_recipe():
    form = RecipeForm()
    if form.validate_on_submit():
        recipe = Recipe(
            title=form.title.data,
            description=form.description.data,
        )
        db.session.add(recipe)
        db.session.commit()
        return redirect(url_for("home"))
    return render_template("new_recipe.html", title="New Recipe", form=form)


@app.route("/register", methods=["GET", "POST"])
def register():
    if current_user.is_authenticated:
        return redirect(url_for("home"))

    form = RegistrationForm()
    if form.validate_on_submit():
        user = User(
            username=form.username.data,
            email=form.email.data,
        )
        user.set_password(form.password.data)
        db.session.add(user)
        db.session.commit()
        return redirect(url_for("home"))
    return render_template("register.html", title="Register", form=form)
