import React from 'react';
import { GoogleLogout } from 'react-google-login';
// require('dotenv').config();
// import dotenv from 'dotenv';
// const clientId = process.env.REACT_APP_Client_ID;
const clientId = '343890256332-otlmb1slng5u0plp32sp2kt2j0tivh74.apps.googleusercontent.com';
const Logout = () => {
    const onSuccess = (res) => {
        console.log("JSR! Logout successful.");
    }
    return (
        <div id='signOutButton'>
            <GoogleLogout
                clientId={clientId}
                buttonText='LogoutJSR'
                onLogoutSuccess={onSuccess}
            />
        </div>
    )
}

export default Logout;