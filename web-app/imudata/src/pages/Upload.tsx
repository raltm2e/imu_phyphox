import React, {useEffect} from 'react';
import Plot from 'react-plotly.js';
import {useNavigate} from 'react-router-dom';
import {FileInput, H1} from "@blueprintjs/core";
import PageHolder from "../components/PageHolder";
import {ImuDataResult} from "../models/imudata";

const Upload = () => {
  const navigate = useNavigate();
  const [imuDataResult, setImuDataResult] = React.useState<ImuDataResult | undefined>(undefined);

  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const fileList = event.target.files;
    if (fileList) {
      const file = fileList[0];
      const reader = new FileReader();
      reader.onload = (e) => {
        if (e.target) {
          const fileContent = e.target.result;
          console.log(fileContent);
          const data = new FormData();
          // @ts-ignore
          data.append('photo', event.target.files[0]);
          data.append('name', 'Test Name');
          data.append('desc', 'Test description');
          fetch('/imudata_file', {
            method: 'POST',
            headers: {
              'Accept': 'application/json',
              'Access-Control-Allow-Origin': '*',
            },
            body: data,
          })
            .then(r => r.json())
            .then(data => setImuDataResult(data as ImuDataResult))
            .catch(e => console.log("error: ", e));
        }
      };
      reader.readAsText(file);
    }
  }

  useEffect(() => {
    navigate('/upload');
  }, [navigate]);

  return (
    <PageHolder>
      <H1>Upload</H1>
      <div>
        <FileInput large text={'Upload your file'} buttonText={'Upload'} onInputChange={handleFileUpload} />
      </div>
      {imuDataResult &&
        <div>
          <div>
              <p>Repetitions: {imuDataResult.repetitions}</p>
              <p>Exercise time: {imuDataResult.spent_time} seconds</p>
              <p>Total distance: {imuDataResult.total_distance} meters</p>
              <p>Spent energy: {imuDataResult.spent_energy} Joules</p>
          </div>
          <div>
            <Plot
              data={[{
                  type: 'scatter',
                  x: imuDataResult.raw_data.map(data => data.time),
                  y: imuDataResult.raw_data.map(data => data.linear_acceleration_z),
                  name: 'linear_acceleration_z'
                },
              ]}
              layout={ {title: 'Raw data'} }
            />
          </div>
          <div>
            <Plot
                data={[{
                  type: 'scatter',
                  x: imuDataResult.processed_data.map(data => data.time),
                  y: imuDataResult.processed_data.map(data => data.energy),
                  name: 'Energy'
                },
                ]}
                layout={ {title: 'Spent energy'} }
            />
          </div>
        </div>
      }
    </PageHolder>
  );
};

export default Upload;
