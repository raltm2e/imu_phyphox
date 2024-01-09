import {useEffect, useState} from 'react';
import {useNavigate} from 'react-router-dom';
import {H1} from "@blueprintjs/core";
import PageHolder from "../components/PageHolder";

const About = () => {
  const navigate = useNavigate();
  const [fetchedData, setFetchedData] = useState('');

  const fetchData = () => {
    fetch('/hello')
        .then(r => r.text())
        .then(text => setFetchedData(text));
  };

  useEffect(() => {
    fetchData();
    navigate('/about');
  }, [navigate]);

  return (
    <PageHolder>
      <H1>About</H1>
      <p>{fetchedData}</p>
    </PageHolder>
  );
};

export default About;
