import psycopg2
from flask import Flask, jsonify, render_template, request

from automation.main import HOSTNAME, DATABASE, USER, PASSWORD, save_all_data

app = Flask(__name__)


@app.route('/', methods=["GET"])
def index():
    conn = psycopg2.connect(host=HOSTNAME, database=DATABASE, user=USER, password=PASSWORD)
    cur = conn.cursor()
    cur.execute("SELECT * FROM processed_data LIMIT 50")
    rows = cur.fetchall()
    cur.close()
    conn.close()
    return jsonify(rows)


@app.route('/upload_raw')
def upload_raw_csv():
    print("uploading")
    return render_template("index.html")


@app.route('/upload_raw/success', methods = ['POST'])
def success():
    if request.method == 'POST':
        f = request.files['file']
        if f.filename.endswith(".csv"):
            print("saving data")
            save_all_data(f, HOSTNAME, DATABASE, USER, PASSWORD)
            print("data saved")
            return render_template("acknowledgment.html", name = f.filename)
    return render_template("error.html", error = "Wrong file type! File must be .csv")


if __name__ == '__main__':
    app.run()
