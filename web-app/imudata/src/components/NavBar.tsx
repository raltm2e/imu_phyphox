import React from 'react';
import {Link} from 'react-router-dom';
import {Alignment, Navbar} from '@blueprintjs/core';

const NavBar = () => {
  return (
    <Navbar fixedToTop className='h-16 flex items-center'>
      <Navbar.Group align={Alignment.LEFT}>
        <Navbar.Heading
          className='text-m font-bold uppercase tracking-wider text-gray-600'
        >This is a navbar</Navbar.Heading>
        <Navbar.Divider/>
        <Link
          role='button'
          className='bp4-button bp4-minimal bp4-icon-document'
          to='/plots'
        >
          Plots
        </Link>
        <Link
          role='button'
          className='bp4-button bp4-minimal bp4-icon-label'
          to='/about'
        >
          About
        </Link>
      </Navbar.Group>
    </Navbar>
  );
};

export default NavBar;
