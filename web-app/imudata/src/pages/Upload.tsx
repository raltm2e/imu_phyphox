import React, {useEffect} from 'react';
import {useNavigate} from 'react-router-dom';
import {FileInput, H1} from "@blueprintjs/core";
import PageHolder from "../components/PageHolder";
import {useAddRawDataMutation} from "../slices/apiSlice";

const Upload = () => {
  const navigate = useNavigate();
  const [ addRawData ] = useAddRawDataMutation();


  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const fileList = event.target.files;
    if (fileList !== null) {
      const file = fileList[0];
      const reader = new FileReader();
      reader.onload = (e) => {
        // @ts-ignore
        const fileContent = e.target.result as string;
        addRawData({fileContent: fileContent});
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
      <FileInput large text={'Please upload your file'} buttonText={'Upload'} onInputChange={handleFileUpload} />
    </PageHolder>
  );
};

export default Upload;
