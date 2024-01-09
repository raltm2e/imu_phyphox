import React, {useEffect} from 'react';
import {Link, useNavigate} from 'react-router-dom';
import {Card, H1} from "@blueprintjs/core";
import PageHolder from "../components/PageHolder";

const Home = () => {
  const navigate = useNavigate();

  useEffect(() => {
    navigate('/');
  }, [navigate]);

  return (
    <PageHolder>
      <Card>
        <H1>Home</H1>
        <Link
          role='button'
          className='bp4-button bp4-minimal bp4-icon-label'
          to='/upload'
        >Upload</Link>
      </Card>
    </PageHolder>
  );
};

export default Home;
