SELECT raw_data.time, linear_acceleration_z, velocity, distance, energy
FROM raw_data
         JOIN processed_data ON raw_data.username = processed_data.username
    AND raw_data.filename = processed_data.filename
    AND raw_data.time = processed_data.time
WHERE raw_data.filename = '/home/robert/IdeaProjects/imu_phyphox/Acceleration without g 2023-03-26 13-07-06/Raw Data.csv'
  AND raw_data.time < 0.1;