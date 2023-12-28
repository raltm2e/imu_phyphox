import React, {useEffect} from 'react';
import {useNavigate} from 'react-router-dom';
import {FileInput, H1} from "@blueprintjs/core";
import PageHolder from "../components/PageHolder";
import {ProcessedData} from "../models/imudata";

const Upload = () => {
  const navigate = useNavigate();
  const [processedData, setProcessedData] = React.useState<ProcessedData | undefined>(undefined);

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
            .then(data => setProcessedData(data as ProcessedData))
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
        <p>Repetitions: {processedData?.repetitions}</p>
        <p>Exercise time: {processedData?.spent_time} seconds</p>
        <p>Total distance: {processedData?.total_distance} meters</p>
        <p>Spent energy: {processedData?.spent_energy} Joules</p>
      </div>
    </PageHolder>
  );
};

export default Upload;
