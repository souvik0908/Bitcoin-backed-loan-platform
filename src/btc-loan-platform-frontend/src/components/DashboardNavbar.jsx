import React from 'react';
import './DashboardNavbar.css';

const DashboardNavbar = ({ principal, onLogout }) => {
  const shortPrincipal = principal
    ? `${principal.slice(0, 5)}...${principal.slice(-5)}`
    : '';

  return (
    <header className="dashboard-navbar">
      <div className="dashboard-logo">BTCollat</div>

      <nav className="dashboard-nav-links">
        <a href="#overview">Overview</a>
        <a href="#loans">My Loans</a>
        <a href="#nfts">NFT Badges</a>
        <a href="#settings">Settings</a>
      </nav>

      <div className="dashboard-user-section">
        <span className="principal-id">PI: {shortPrincipal}</span>
        <button className="logout-btn" onClick={onLogout}>Logout</button>
      </div>
    </header>
  );
};

export default DashboardNavbar;
