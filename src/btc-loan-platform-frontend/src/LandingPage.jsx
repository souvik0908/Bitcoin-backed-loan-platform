import React from 'react';
import './LandingPage.css';


const LandingPage = ({ onGetStarted }) => {
    return (
        <div className="landing-wrapper">
            <header className="site-header">
                <div className="logo">BTCollat</div>
                <nav className="nav-links">
                    <a href="#home">Home</a>
                    <a href="#how-it-works">How It Works</a>
                    <a href="#nfts">NFT Rewards</a>
                    <a href="#contact">Contact</a>
                </nav>
            </header>

            <section className="landing" id="home">
                <div className="landing-left">
                    <h1>
                        <span className="highlight">BTC Collateral Hub</span>
                    </h1>
                    <p className="tagline">
                        Unlock loans against your Bitcoin without selling. Fast. Secure. Decentralized.
                    </p>

                    <ul className="features-list">
                        <li> Bitcoin-backed Loans via ICP</li>
                        <li> No KYC — DeFi freedom</li>
                        <li> NFT Badge Rewards Coming Soon</li>
                    </ul>

                    <button className="start-btn" onClick={onGetStarted}>
                        🚀 Connect Wallet & Start
                    </button>
                </div>

                <div className="landing-right">
                    <img src="/Bitcoin.svg" alt="Bitcoin" className="spinning-coin" />
                </div>
            </section>

            <section className="description-section" id="how-it-works">
                <h2>How the Platform Works</h2>
                <p>
                    BTC Collateral Hub allows users to take out secure loans against their Bitcoin holdings.
                    Your BTC is locked in a smart contract through ICP's native Bitcoin integration,
                    ensuring full transparency and trustless operations.
                    Once deposited, you receive a stable token loan without any need to sell your crypto or go through tedious KYC checks.
                </p>

                <p>
                    In the near future, we’ll introduce **NFT-based badges** to reward responsible borrowers and long-term participants.
                    These badges will unlock exclusive benefits within the ecosystem.
                </p>

                <p>
                    Our mission is simple: empower Bitcoin holders with decentralized finance tools that are fast, fair, and future-ready.
                </p>
            </section>


            <footer className="site-footer" id="contact">
                <p>© {new Date().getFullYear()} BTC Collateral Hub. All rights reserved.</p>
                <p>📧 support@btccollateralhub.com</p>
            </footer>

        </div>
    );
};

export default LandingPage;
