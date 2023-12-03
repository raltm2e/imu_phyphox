import {useEffect} from 'react';
import {useNavigate} from 'react-router-dom';
import {H1} from "@blueprintjs/core";

const About = () => {
  const navigate = useNavigate();

  useEffect(() => {
    navigate('/about');
  }, [navigate]);

  return (
    <div>
      <H1>About</H1>
    </div>
  );
};

export default About;
