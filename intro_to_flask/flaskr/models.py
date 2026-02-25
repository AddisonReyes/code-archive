from flask_wtf import FlaskForm
from wtforms import BooleanField, IntegerField, StringField, SubmitField, TextAreaField
from wtforms.validators import InputRequired, Length

from . import db


class Task(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    title = db.Column(db.String(100), nullable=False)
    description = db.Column(db.String(200), nullable=True)
    is_complete = db.Column(db.Boolean, nullable=False)
    priority = db.Column(db.Integer, default=0, nullable=True)


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
    priority = IntegerField("Priority", validators=[InputRequired()])

    submit = SubmitField("Submit")
