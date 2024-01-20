import React, {useState, useEffect} from 'react';
import Plot from 'react-plotly.js';
import {useNavigate} from 'react-router-dom';
import {Card, FileInput, H1, InputGroup} from "@blueprintjs/core";
import PageHolder from "../components/PageHolder";
import {ImuDataResult} from "../models/imudata";
import styles from '../styles/Upload.module.css';

const Upload = () => {
  const navigate = useNavigate();
  const [imuDataResult, setImuDataResult] = useState<ImuDataResult | undefined>(undefined);
  const [massParameter, setMassParameter] = useState('');

  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    setImuDataResult(undefined);
    const fileList = event.target.files;
    if (fileList) {
      const file = fileList[0];
      const reader = new FileReader();
      reader.onload = (e) => {
        if (e.target && event.target.files) {
          const data = new FormData();
          data.append('photo', event.target.files[0]);
          data.append('name', 'Test Name');
          data.append('desc', 'Test description');
          fetch(`/imudata_file/${massParameter}`, {
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
      <Card className={styles.card}>
        <H1>Upload</H1>
        <div>
          <InputGroup large type="number" value={massParameter} onChange={e => setMassParameter(e.target.value)}
                 placeholder="Mass of weights"/>
        </div>
        <br/>
        <div>
          <FileInput large fill text={'Upload csv file'} buttonText={'Upload'} onInputChange={handleFileUpload}/>
        </div>
      </Card>
      <br/>
      {imuDataResult &&
          <Card className={styles.uploadCard}>
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
                    layout={{title: 'Raw data'}}
                />
              </div>
              <div>
                <Plot
                    data={[{
                      type: 'scatter',
                      x: imuDataResult.processed_data.map(data => data.time),
                      y: imuDataResult.processed_data.map(data => data.velocity),
                      name: 'distance'
                    },
                    ]}
                    layout={{title: 'Velocity'}}
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
                    layout={{title: 'Spent energy'}}
                />
              </div>
            </div>
          </Card>
        }
    </PageHolder>
  );
};

export default Upload;
