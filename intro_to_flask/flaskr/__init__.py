from flask import Flask
from flask_migrate import Migrate
from flask_sqlalchemy import SQLAlchemy

# print(os.urandom(24).hex())
SECRET_KEY = "e77ef067f5a4882c4821cec3cafc7001adc249327406890e"

app = Flask(__name__)
app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///database.db"
app.config["SECRET_KEY"] = SECRET_KEY
db = SQLAlchemy(app)
migrate = Migrate(app, db)


@app.cli.command("init-db")
def init_db():
    db.create_all()
    print("Initialized the database.")


from . import models, routes
