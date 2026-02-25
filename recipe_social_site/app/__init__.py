from flask import Flask
from flask_migrate import Migrate
from flask_sqlalchemy import SQLAlchemy

app = Flask(__name__)
app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///database.db"
app.config["SECRET_KEY"] = "ecbe91bafa0fb5aea888f3ad22dec1f4cf4e9001ebe23c2c"

db = SQLAlchemy(app)
migrate = Migrate(app, db)

from app import models, routes  # noqa
