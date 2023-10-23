from pandas import read_csv
from automation.data_processing import process_data
from automation.db_integration import insert_csv_into_db, insert_df_into_db
HOSTNAME = "localhost"
DATABASE = "postgres"
USER = "postgres"
PASSWORD = "pass"

FILENAME = "/home/robert/IdeaProjects/imu_phyphox/Acceleration without g 2023-03-26 13-07-06/Raw Data.csv"


def save_all_data(file, mass, hostname, database, user, password):
    df_raw = read_csv(file)
    try:
        insert_csv_into_db(df_raw, file, hostname, database, user, password)
    finally:
        df_summary = process_data(df_raw, mass)
        print(df_summary.head(10))
        try:
            insert_df_into_db(df_summary, file, hostname, database, user, password)
        finally:
            return df_summary
