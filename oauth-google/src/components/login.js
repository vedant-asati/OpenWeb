import React from 'react';
import { GoogleLogin } from 'react-google-login';
// require('dotenv').config();
// import dotenv from 'dotenv';
// const clientId = process.env.REACT_APP_Client_ID;
const clientId = '343890256332-otlmb1slng5u0plp32sp2kt2j0tivh74.apps.googleusercontent.com';
const Login = () => {
    const onSuccess = (res) => {
        console.log("JSR! Login success for : ", res);
    }
    const onFailure = (res) => {
        console.log("JSR! Login failed : ", res);
    }
    return (
        <div id='signInButton'>
            <GoogleLogin
                clientId={clientId}
                buttonText='LoginJSR'
                onSuccess={onSuccess}
                onFailure={onFailure}
                cookiePolicy={'single_host_origin'}
                isSignedIn={true}
            />
        </div>
    )
}

export default Login;