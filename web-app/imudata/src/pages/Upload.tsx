import React, {useState, useEffect} from 'react';
import Plot from 'react-plotly.js';
import {useNavigate} from 'react-router-dom';
import {Button, Card, FileInput, H1, H5, InputGroup, MenuItem} from "@blueprintjs/core";
import { ItemRenderer, Select } from "@blueprintjs/select";
import PageHolder from "../components/PageHolder";
import {ImuDataResult, Noise} from "../models/imudata";
import styles from '../styles/Upload.module.css';

const Upload = () => {
  const navigate = useNavigate();
  const [imuDataResult, setImuDataResult] = useState<ImuDataResult | undefined>(undefined);
  const [massParameter, setMassParameter] = useState('');
  const [noiseParameter, setNoiseParameter] = useState(Noise.LOW);
  const NoiseSelect = Select<Noise>;
  const renderNoise: ItemRenderer<Noise> = (noise, { handleClick, modifiers }) => {
    if (!modifiers.matchesPredicate) {
      return null;
    }
    return (
        <MenuItem
            active={modifiers.active}
            key={noise}
            onClick={handleClick}
            text={noise}
        />
    );
  };

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
          fetch(`/imudata_file/${massParameter}&${noiseParameter}`, {
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
      <Card className={styles.card}>
        <div>
          <H5>Mass (kg)</H5>
          <InputGroup large type="number" value={massParameter} onChange={e => setMassParameter(e.target.value)}
                      placeholder="Mass (kg)"/>
        </div>
        <br/>
        <div>
          <H5>Noise level</H5>
          <NoiseSelect
              fill
              filterable={false}
              items={Object.values(Noise)}
              itemRenderer={renderNoise}
              onItemSelect={setNoiseParameter}
              inputProps={{disabled: true}}
          >
            <Button text={noiseParameter} rightIcon="double-caret-vertical" />
          </NoiseSelect>
        </div>
        <br/>
        <div>
          <H5>Raw data</H5>
          <FileInput large fill text={'Upload csv file'} buttonText={'Upload'} onInputChange={handleFileUpload}/>
        </div>
      </Card>
      <br/>
      {imuDataResult &&
          <Card className={styles.card}>
            <div>
              <div>
                <p>Repetitions: {imuDataResult.repetitions}</p>
                <p>Mass: {imuDataResult.mass} kg</p>
                <p>Exercise time: {imuDataResult.spent_time} seconds</p>
                <p>Total distance: {imuDataResult.total_distance} meters</p>
                <p>Spent energy: {imuDataResult.spent_energy} kcal</p>
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
