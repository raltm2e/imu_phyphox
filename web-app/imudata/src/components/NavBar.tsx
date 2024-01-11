import React from 'react';
import {Link} from 'react-router-dom';
import {Alignment, Navbar, NavbarHeading} from '@blueprintjs/core';
import styles from '../styles/Navbar.module.css';

const NavBar = () => {
  return (
    <Navbar fixedToTop className={styles.navbar}>
      <Navbar.Group align={Alignment.LEFT}>
        <NavbarHeading
          className='text-m font-bold uppercase tracking-wider text-gray-600'
        >IMU data</NavbarHeading>
        <Navbar.Divider/>
        <NavbarHeading>
          <Link
              role='button'
              className='bp4-button bp4-minimal bp4-icon-label'
              to='/upload'
          >Upload</Link>
        </NavbarHeading>
        <NavbarHeading>
          <Link
              role='button'
              className='bp4-button bp4-minimal bp4-icon-label'
              to='/about'
          >About</Link>
        </NavbarHeading>
      </Navbar.Group>
    </Navbar>
  );
};

export default NavBar;
