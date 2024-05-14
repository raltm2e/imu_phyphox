# Overview

This is a repository that contains 
- smartphone's acceleration logs from a PhyPhox iOS application
- data processing implemented in a Jupyter Notebook
- web server implemented in Rust
- user interface implemented using React

## Jupyter

The `acceleration_analysis.ipynb` was run with Python 3.9.

## Web server

The web server in `/imu-backend` was implemented using version 1.72 of Rust. Simply write 

`cargo run` 

to start the service.

## User interface

The user interface is implemented using the React framework and Yarn package manager. Write 

`yarn && yarn start`

to start the interface.
