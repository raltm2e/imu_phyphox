import {useEffect} from 'react';
import {useNavigate} from 'react-router-dom';
import {H1} from "@blueprintjs/core";

const Plots = () => {
  const navigate = useNavigate();

  useEffect(() => {
    navigate('/plots');
  }, [navigate]);

  return (
    <div>
      <H1>Plots</H1>
    </div>
  );
};

export default Plots;
