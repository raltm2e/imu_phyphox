import {useEffect} from 'react';
import {useNavigate} from 'react-router-dom';
import {H1} from "@blueprintjs/core";

const Home = () => {
  const navigate = useNavigate();

  useEffect(() => {
    navigate('/');
  }, [navigate]);

  return (
    <div>
      <H1>Home</H1>
    </div>
  );
};

export default Home;
