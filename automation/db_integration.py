import psycopg2
import csv

def insert_csv_into_db(filename: str):
    conn = psycopg2.connect(host="localhost", database="postgres", user="postgres", password="pass")

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
