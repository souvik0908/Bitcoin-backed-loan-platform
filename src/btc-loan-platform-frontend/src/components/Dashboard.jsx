// import React from 'react';
// import DashboardNavbar from "./DashboardNavbar";


// const Dashboard = ({ principal, onLogout }) => {
//   const shortPrincipal = principal
//     ? `${principal.slice(0, 5)}...${principal.slice(-5)}`
//     : '';

//   return (
//     <>
//       <DashboardNavbar principal={principal} onLogout={onLogout} />
//       <div style={{ padding: '2rem', textAlign: 'center' }}>
//         <h2> Welcome, {shortPrincipal}</h2>
//         <p>This is dashboard area. More features coming soon!</p>
//       </div>
//     </>
//   );
// };

// export default Dashboard;

import React, { useEffect, useState } from "react";
import DashboardNavbar from "./DashboardNavbar";

// Import canisters
import { btc_token_canister } from "../../../declarations/btc_token_canister";
import { iusd_token_canister } from "../../../declarations/iusd_token_canister";
import { loan_canister } from "../../../declarations/loan_canister";
import { oracle_canister } from "../../../declarations/oracle_canister";


const Dashboard = ({ principal, onLogout }) => {
  const [btcBalance, setBtcBalance] = useState(0);
  const [iusdBalance, setIusdBalance] = useState(0);
  const [btcPrice, setBtcPrice] = useState(0);
  const [loanInfo, setLoanInfo] = useState({});
  const [borrowAmount, setBorrowAmount] = useState("");

  const shortPrincipal = principal
    ? `${principal.slice(0, 5)}...${principal.slice(-5)}`
    : "";

  // Fetch balances, BTC price, and loan info
  useEffect(() => {
    const fetchData = async () => {
      try {
        const [btc, iusd, price, loan] = await Promise.all([
          btc_token_canister.get_balance(), // replace with actual method
          iusd_token_canister.get_balance(),
          oracle_canister.get_price(),
          loan_canister.get_loan_info(),
        ]);

        setBtcBalance(Number(btc));
        setIusdBalance(Number(iusd));
        setBtcPrice(Number(price));
        setLoanInfo(loan);
      } catch (err) {
        console.error("Error fetching dashboard data:", err);
      }
    };

    fetchData();
  }, []);

  // Handle borrowing
  const handleBorrow = async () => {
    try {
      const res = await loan_canister.borrow(BigInt(borrowAmount));
      alert("Loan successful!");
      setBorrowAmount("");
    } catch (err) {
      console.error(err);
      alert("Loan failed.");
    }
  };

  return (
    <div className="min-h-screen bg-[#0d0d0d] text-white">
      <DashboardNavbar principal={principal} onLogout={onLogout} />

      <main className="p-6 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {/* BTC Balance */}
        <div className="bg-[#1e1e1e] p-4 rounded-xl shadow-lg">
          <h2 className="text-xl font-semibold mb-2">BTC Balance</h2>
          <p className="text-2xl font-mono">{btcBalance} BTC</p>
        </div>

        {/* iUSD Balance */}
        <div className="bg-[#1e1e1e] p-4 rounded-xl shadow-lg">
          <h2 className="text-xl font-semibold mb-2">iUSD Balance</h2>
          <p className="text-2xl font-mono">{iusdBalance} iUSD</p>
        </div>

        {/* BTC Price */}
        <div className="bg-[#1e1e1e] p-4 rounded-xl shadow-lg">
          <h2 className="text-xl font-semibold mb-2">BTC Price (Oracle)</h2>
          <p className="text-2xl font-mono">${btcPrice}</p>
        </div>

        {/* Loan Form */}
        <div className="col-span-1 md:col-span-2 bg-[#1e1e1e] p-6 rounded-xl shadow-lg">
          <h2 className="text-xl font-semibold mb-4">Borrow iUSD</h2>
          <div className="flex flex-col md:flex-row gap-4">
            <input
              type="number"
              placeholder="Amount in BTC"
              className="p-2 rounded-lg bg-[#2c2c2c] text-white flex-1"
              value={borrowAmount}
              onChange={(e) => setBorrowAmount(e.target.value)}
            />
            <button
              className="bg-purple-600 hover:bg-purple-700 px-6 py-2 rounded-lg"
              onClick={handleBorrow}
            >
              Borrow
            </button>
          </div>
        </div>

        {/* Loan Info */}
        <div className="col-span-1 md:col-span-3 bg-[#1e1e1e] p-6 rounded-xl shadow-lg">
          <h2 className="text-xl font-semibold mb-4">Loan Summary</h2>
          <pre className="bg-[#2c2c2c] p-4 rounded-md overflow-auto text-sm">
            {JSON.stringify(loanInfo, null, 2)}
          </pre>
        </div>
      </main>
    </div>
  );
};

export default Dashboard;

