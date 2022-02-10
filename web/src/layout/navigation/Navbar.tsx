import { Link } from 'react-router-dom';

import logo from '../../media/clomonitor.svg';
import Searchbar from '../common/Searchbar';
import ThemeSwitch from '../common/ThemeSwitch';
import styles from './Navbar.module.css';

const Navbar = () => {
  return (
    <nav className={`navbar ${styles.navbar}`}>
      <div className="container-lg">
        <div className="d-flex flex-column flex-md-row align-items-center justify-content-between w-100">
          <div className="me-0 me-md-4 mt-2 mt-md-0">
            <div className="d-flex flex-row align-items-start">
              <Link to="/" className="cursorPointer">
                <img className={styles.logo} alt="Logo" src={logo} />
              </Link>
              <div className={`ms-1 badge rounded-0 text-uppercase ${styles.badge}`}>Alpha</div>
            </div>
          </div>
          <div className="d-flex flex-row align-items-center">
            <Searchbar classNameWrapper="my-3" />
            <ThemeSwitch />
          </div>
        </div>
      </div>
    </nav>
  );
};

export default Navbar;