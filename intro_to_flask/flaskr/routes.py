from flask import redirect, render_template, url_for

from . import app, db
from .models import Task, TaskForm


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


@app.route("/task/update/<int:task_id>", methods=["POST"])
def update_task_status(task_id):
    task = Task.query.get_or_404(task_id)
    if task:
        task.is_complete = not task.is_complete
        db.session.commit()
    return redirect(url_for("display_tasks"))


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
