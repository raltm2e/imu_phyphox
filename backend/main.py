from flask import Flask, render_template, request

from backend.automation.data_processing import get_plots
from backend.automation.db_integration import save_all_data
HOSTNAME = "localhost"
DATABASE = "postgres"
USER = "postgres"
PASSWORD = "pass"

app = Flask(__name__)


@app.route('/', methods=["GET"])
def index():
    return render_template("index.html")


@app.route('/success', methods=['POST'])
def success():
    if request.method == 'POST':
        f = request.files['file']
        mass = int(request.form.get("mass"))
        if f.filename.endswith(".csv"):
            processed_df = save_all_data(f, mass, HOSTNAME, DATABASE, USER, PASSWORD)
            generated_plot = get_plots(processed_df)
            return render_template("acknowledgment.html", name=f.filename, image=generated_plot)
        return render_template("error.html", error="Wrong file type! File must be .csv")
    return render_template("error.html", error="Bad request")


if __name__ == '__main__':
    app.run(host='0.0.0.0', debug=False)
