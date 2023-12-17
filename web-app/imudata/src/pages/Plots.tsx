import {useEffect} from 'react';
import {useNavigate} from 'react-router-dom';
import {H1} from "@blueprintjs/core";
import PageHolder from "../components/PageHolder";

const Plots = () => {
  const navigate = useNavigate();

  useEffect(() => {
    navigate('/plots');
  }, [navigate]);

  return (
    <PageHolder>
      <H1>Plots</H1>
    </PageHolder>
  );
};

export default Plots;
