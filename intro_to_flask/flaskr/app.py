from flask import Flask, render_template
from flask_migrate import Migrate
from flask_sqlalchemy import SQLAlchemy

app = Flask(__name__)
app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///database.db"
app.config["SECRET_KEY"] = "e77ef067f5a4882c4821cec3cafc7001adc249327406890e"
db = SQLAlchemy(app)
migrate = Migrate(app)


class Task(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(100), nullable=False)
    description = db.Column(db.String(200), nullable=True)
    is_complete = db.Column(db.Boolean, nullable=False)


@app.cli.command("init-db")
def init_db():
    db.create_all()
    migrate
    print("Initialized the database.")


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
