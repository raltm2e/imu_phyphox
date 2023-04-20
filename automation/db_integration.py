import psycopg2
import csv
from sqlalchemy import create_engine
from pandas import DataFrame


def insert_csv_into_db(filename: str, hostname: str, database: str, user: str, password: str):
	conn = psycopg2.connect(host=hostname, database=database, user=user, password=password)

	with open(filename, 'r') as f:
		reader = csv.reader(f)
		# Skip the header row
		next(reader)

		for row in reader:
			time = float(row[0])
			linear_acceleration_x = float(row[1])
			linear_acceleration_y = float(row[2])
			linear_acceleration_z = float(row[3])
			absolute_acceleration = float(row[4])

			cur = conn.cursor()
			cur.execute("INSERT INTO raw_data (username, filename, time, linear_acceleration_x, linear_acceleration_y, linear_acceleration_z, absolute_acceleration) VALUES (%s, %s, %s, %s, %s, %s, %s)", ('user1', filename, time, linear_acceleration_x, linear_acceleration_y, linear_acceleration_z, absolute_acceleration))
			conn.commit()
			cur.close()
	conn.close()


def insert_df_into_db(df_summary: DataFrame, filename: str, hostname: str, database: str, user: str, password: str):
	conn = psycopg2.connect(
		host=hostname,
		database=database,
		user=user,
		password=password,
	)
	engine = create_engine('postgresql://' + user + ':' + password + '@' + hostname + '/' + database)
	df_summary["username"] = "user1"
	df_summary["filename"] = filename
	df_summary = df_summary[['username', 'filename', 'Time (s)', 'Velocity (m/s)', 'Distance (m)', 'Energy (J)']]
	df_summary = df_summary.rename(columns={'Time (s)': 'time', 'Velocity (m/s)': 'velocity', 'Distance (m)': 'distance',
	                                        'Energy (J)': 'energy'})
	df_summary.to_sql('processed_data', engine, if_exists='append', index=False)
	conn.close()
