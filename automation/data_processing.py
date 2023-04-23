import base64
import copy
from io import BytesIO

from matplotlib import pyplot as plt
from matplotlib.backends.backend_agg import FigureCanvasAgg as FigureCanvas
from pandas import *

ACCELERATION_NOISE_THRESHOLD_POSITIVE = 0.9
ACCELERATION_NOISE_THRESHOLD_NEGATIVE = -1.5
MASS = 5  # body mass in kg

def filter_noise(dataframe):
    new_df = copy.deepcopy(dataframe)
    changes = new_df.pct_change()["Linear Acceleration z (m/s^2)"]
    for i, change in changes.items():
        if change > ACCELERATION_NOISE_THRESHOLD_POSITIVE or change < ACCELERATION_NOISE_THRESHOLD_NEGATIVE:
            new_df["Linear Acceleration z (m/s^2)"][i] = 0.0
    new_df.plot.line(x="Time (s)", y="Linear Acceleration z (m/s^2)", figsize=(22, 9))
    return new_df


def get_velocity(acceleration: float, V0: float, delta_t: float):
    # a = (V1-V0) / t
    # V1 = a*t + V0
    return acceleration * delta_t + V0


def get_distance(delta_V: float, delta_t: float):
    # V = s/t
    # s = V*t
    return delta_V * delta_t


def get_energy_spent(mass: float, distance: float, acceleration: float):
    # A = F*s
    # F = m*a
    # A = m*a*s
    return mass * acceleration * distance


def process_data(file) -> DataFrame:
    df = read_csv(file)
    previous_time = 0.0
    previous_velocity = 0.0
    total_distance = 0.0
    total_energy = 0.0
    velocity_vec = [[]]
    distance_vec = [[]]
    energy_vec = [[]]
    for index, row in df.iterrows():
        timestep = row["Time (s)"] - previous_time

        velocity = get_velocity(row["Linear Acceleration z (m/s^2)"], previous_velocity, timestep)
        velocity_vec.append([row["Time (s)"], velocity])

        distance_step = abs(get_distance(velocity, timestep))
        total_distance += distance_step
        distance_vec.append([row["Time (s)"], total_distance])

        energy_step = abs(get_energy_spent(MASS, distance_step, row["Linear Acceleration z (m/s^2)"]))
        total_energy += energy_step
        energy_vec.append([row["Time (s)"], total_energy])

        previous_velocity = velocity
        previous_time = row["Time (s)"]

    df_velocity = DataFrame(velocity_vec, columns=["Time (s)", "Velocity (m/s)"])
    df_distance = DataFrame(distance_vec, columns=["Time (s)", "Distance (m)"])
    df_energy = DataFrame(energy_vec, columns=["Time (s)", "Energy (J)"])
    print("Distance is (m): ", total_distance)
    print("Energy is (J): ", total_energy)

    df_summary = df_velocity.merge(df_distance, on="Time (s)").merge(df_energy, on="Time (s)").dropna()
    return df_summary


def get_plots(processed_df: DataFrame):
    fig, axis = plt.subplots()
    processed_df.plot.line(x="Time (s)", y="Energy (J)", ax=axis)
    processed_df.plot.line(x="Time (s)", y="Distance (m)", ax=axis)
    processed_df.plot.line(x="Time (s)", y="Velocity (m/s)", ax=axis)
    # TODO subplots, y-axis
    axis.set_title("Exercise summary")
    axis.set_xlabel("Time (s)")
    axis.grid()
    axis.plot()

    # Convert plot to PNG image
    pngImage = BytesIO()
    FigureCanvas(fig).print_png(pngImage)

    # Encode PNG image to base64 string
    pngImageB64String = "data:image/png;base64,"
    pngImageB64String += base64.b64encode(pngImage.getvalue()).decode('utf8')
    return pngImageB64String
