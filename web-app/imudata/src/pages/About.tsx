import {useEffect} from 'react';
import {useNavigate} from 'react-router-dom';
import {H1} from "@blueprintjs/core";
import PageHolder from "../components/PageHolder";

const About = () => {
  const navigate = useNavigate();

  useEffect(() => {
    navigate('/about');
  }, [navigate]);

  return (
    <PageHolder>
      <H1>About</H1>
    </PageHolder>
  );
};

export default About;
