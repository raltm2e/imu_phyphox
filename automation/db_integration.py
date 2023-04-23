import psycopg2
from sqlalchemy import create_engine
from pandas import DataFrame


def insert_csv_into_db(df_raw: DataFrame, file, hostname: str, database: str, user: str, password: str):
    conn = psycopg2.connect(
        host=hostname,
        database=database,
        user=user,
        password=password
    )
    engine = create_engine('postgresql://' + user + ':' + password + '@' + hostname + '/' + database)
    df_raw["username"] = "user1"
    df_raw["filename"] = file.filename

    df_raw = df_raw[['username', 'filename', 'Time (s)', 'Linear Acceleration x (m/s^2)',
                             'Linear Acceleration y (m/s^2)', 'Linear Acceleration z (m/s^2)',
                             'Absolute acceleration (m/s^2)']]
    df_raw = df_raw.rename(columns={'Time (s)': 'time', 'Linear Acceleration x (m/s^2)': 'linear_acceleration_x',
                                            'Linear Acceleration y (m/s^2)': 'linear_acceleration_y',
                                            'Linear Acceleration z (m/s^2)': 'linear_acceleration_z',
                                            'Absolute acceleration (m/s^2)': 'absolute_acceleration',
                                            })
    df_raw.to_sql('raw_data', engine, if_exists='append', index=False)
    conn.close()


def insert_df_into_db(df_summary: DataFrame, file, hostname: str, database: str, user: str, password: str):
    conn = psycopg2.connect(
        host=hostname,
        database=database,
        user=user,
        password=password,
    )
    engine = create_engine('postgresql://' + user + ':' + password + '@' + hostname + '/' + database)
    df_summary["username"] = "user1"
    df_summary["filename"] = file.filename
    df_summary = df_summary[['username', 'filename', 'Time (s)', 'Velocity (m/s)', 'Distance (m)', 'Energy (J)']]
    df_summary = df_summary.rename(columns={'Time (s)': 'time', 'Velocity (m/s)': 'velocity', 'Distance (m)': 'distance',
                                            'Energy (J)': 'energy'})
    df_summary.to_sql('processed_data', engine, if_exists='append', index=False)
    conn.close()
