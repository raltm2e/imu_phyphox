from automation.data_processing import process_data
from automation.db_integration import insert_csv_into_db, insert_df_into_db
HOSTNAME = "localhost"
DATABASE = "postgres"
USER = "postgres"
PASSWORD = "pass"

FILENAME = "/home/robert/IdeaProjects/imu_phyphox/Acceleration without g 2023-03-26 13-07-06/Raw Data.csv"


if __name__ == '__main__':
    insert_csv_into_db(FILENAME, HOSTNAME, DATABASE, USER, PASSWORD)
    df_summary = process_data(FILENAME)
    print(df_summary.head(10))
    insert_df_into_db(df_summary, FILENAME, HOSTNAME, DATABASE, USER, PASSWORD)
