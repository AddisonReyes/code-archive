from flask import Flask, redirect, render_template, url_for
from flask_migrate import Migrate
from flask_sqlalchemy import SQLAlchemy
from flask_wtf import FlaskForm
from wtforms import BooleanField, StringField, SubmitField, TextAreaField
from wtforms.validators import InputRequired, Length

# print(os.urandom(24).hex())
SECRET_KEY = "e77ef067f5a4882c4821cec3cafc7001adc249327406890e"

app = Flask(__name__)
app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///database.db"
app.config["SECRET_KEY"] = SECRET_KEY
db = SQLAlchemy(app)
migrate = Migrate(app, db)


class Task(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(100), nullable=False)
    description = db.Column(db.String(200), nullable=True)
    is_complete = db.Column(db.Boolean, nullable=False)


class TaskForm(FlaskForm):
    title = StringField(
        "Title",
        validators=[InputRequired(), Length(min=1, max=100)],
    )
    description = TextAreaField(
        "Description",
        validators=[Length(max=200)],
    )
    is_complete = BooleanField("Is Complete")
    submit = SubmitField("Submit")


@app.cli.command("init-db")
def init_db():
    db.create_all()
    print("Initialized the database.")


@app.route("/task", methods=["GET", "POST"])
def task():
    form = TaskForm()
    if form.validate_on_submit():
        new_task = Task(
            title=form.title.data,
            description=form.description.data,
            is_complete=form.is_complete.data,
        )
        db.session.add(new_task)
        db.session.commit()
        return redirect(url_for("display_tasks"))
    return render_template("task.html", form=form)


@app.route("/task/list/")
def display_tasks():
    tasks = Task.query.all()
    return render_template("task_list.html", tasks=tasks)


@app.route("/")
def hello_world():
    return "Hello World!"


@app.route("/user/<name>", methods=["GET"])
def user(name):
    personal = f"<h1>Hello, {name}</h1>"
    instruc = """
        <p>
            Change the name in the <em>browser address bar</em>
            and reload the page
        </p>
    """
    return personal + instruc


@app.route("/hello/<name>")
def hello(name):
    return render_template("hello.html", name=name)


@app.route("/users")
def users():
    user_names = ["Alice", "Bob", "Charlie"]
    return render_template("users.html", names=user_names)


if __name__ == "__main__":
    app.run(debug=True)
