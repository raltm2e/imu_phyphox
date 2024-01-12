import {useEffect, useState} from 'react';
import {useNavigate} from 'react-router-dom';
import {Card, H1} from "@blueprintjs/core";
import PageHolder from "../components/PageHolder";
import styles from '../styles/Upload.module.css';

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
      <Card className={styles.card}>
        <H1>About</H1>
        <p>{fetchedData}</p>
      </Card>
    </PageHolder>
  );
};

export default About;
