CREATE TABLE raw_data (
    username VARCHAR(30) NOT NULL,
    filename VARCHAR(100) NOT NULL,
    time FLOAT NOT NULL,
    linear_acceleration_x FLOAT NOT NULL,
    linear_acceleration_y FLOAT NOT NULL,
    linear_acceleration_z FLOAT NOT NULL,
    absolute_acceleration FLOAT NOT NULL,
    PRIMARY KEY(username, filename, time)
);

CREATE TABLE processed_data (
    username VARCHAR(30) NOT NULL,
    filename VARCHAR(100) NOT NULL,
    time FLOAT NOT NULL,
    velocity FLOAT NOT NULL,
    distance FLOAT NOT NULL,
    energy FLOAT NOT NULL,
    PRIMARY KEY(username, filename, time),
    FOREIGN KEY (username, filename, time) REFERENCES raw_data (username, filename, time)
);