// import { useState, useEffect } from 'react';
// import LandingPage from './LandingPage';
// import { useAuth } from './auth/AuthProvider';
// import Dashboard from './components/Dashboard';


// function App() {
//   const { isAuthenticated, login, logout, principal } = useAuth();
//   const [step, setStep] = useState('landing');

//   // Move to dashboard (or authenticated step) when logged in
//   useEffect(() => {
//     if (isAuthenticated) {
//       setStep('dashboard'); // or any authenticated step name
//     }
//   }, [isAuthenticated]);

//   const handleGetStarted = () => {
//     if (!isAuthenticated) {
//       login();
//     } else {
//       setStep('dashboard');
//     }
//   };

//   function shortenPrincipal(principal) {
//   return principal
//     ? `${principal.slice(0, 5)}...${principal.slice(-5)}`
//     : '';
// }

//   return (
//     <>
//       {step === 'landing' && <LandingPage onGetStarted={handleGetStarted} />}

//       {step === 'dashboard' && (
//         <div style={{ textAlign: 'center', padding: '4rem' }}>
//           <h2>Welcome, {shortenPrincipal(principal)}</h2>
//           <button onClick={() => { logout(); setStep('landing'); }}>
//             Logout
//           </button>
//           {/* 
//             Here you can replace with your Dashboard component later 
//           */}
//           <p>This is your dashboard area. More features coming soon!</p>
//         </div>
//       )}
//     </>
//   );
// }

// export default App;

import { useState, useEffect } from 'react';
import LandingPage from './LandingPage';
import { useAuth } from './auth/AuthProvider';
import Dashboard from './components/Dashboard';

function App() {
  const { isAuthenticated, login, logout, principal } = useAuth();
  const [step, setStep] = useState('landing');

  useEffect(() => {
    if (isAuthenticated) {
      setStep('dashboard');
    }
  }, [isAuthenticated]);

  const handleGetStarted = () => {
    if (!isAuthenticated) {
      login();
    } else {
      setStep('dashboard');
    }
  };

  return (
    <>
      {step === 'landing' && <LandingPage onGetStarted={handleGetStarted} />}

      {step === 'dashboard' && (
        <Dashboard
          principal={principal}
          onLogout={() => {
            logout();
            setStep('landing');
          }}
        />
      )}
    </>
  );
}

export default App;
