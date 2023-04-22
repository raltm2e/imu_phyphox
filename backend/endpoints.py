import psycopg2
from flask import Flask, jsonify

from automation.main import HOSTNAME, DATABASE, USER, PASSWORD

app = Flask(__name__)


@app.route('/')
def index():
    conn = psycopg2.connect(host=HOSTNAME, database=DATABASE, user=USER, password=PASSWORD)
    cur = conn.cursor()
    cur.execute("SELECT * FROM processed_data WHERE time < 0.05")
    rows = cur.fetchall()
    cur.close()
    conn.close()
    return jsonify(rows)


if __name__ == '__main__':
    app.run()
