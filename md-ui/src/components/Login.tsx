'use client';

import React, { useState } from 'react';

// import { nanoid_gen } from "../../../backend/pkg";
// import { CreateSurveyRequest } from "../../../server/bindings/CreateSurveyRequest";
// import { supabase } from "../supabase";

export default function Login() {
    // const [email, setEmail] = useState('');
    // const [password, setPassword] = useState('');
    const [loggedin, setLoggedIn] = useState(false);

    // const handleEmailChange = (event) => {
    //     setEmail(event.target.value);
    // }

    // const handlePasswordChange = (event) => {
    //     setPassword(event.target.value);
    // }

    const login = (event) => {
        event.preventDefault();
        console.log('handling submit');
        // const data = {
        //     email: email,
        //     password: password
        // };
        console.log(event.target.id);
        const data = {
            email: event.target.email.value,
            password: event.target.password.value
        };
        // const url = event.target.id == 'signup' ? 'signup' : 'login';

        let results = fetch(`http://localhost:8080/login`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            credentials: "include",
            body: JSON.stringify(data)
        })
            .then(response => response.json())
            .then(data => {
                console.log(data);
                // setLoggedIn(true);
            })
            .catch(error => {
                console.error(error);
            });
    }

    const handleSubmit = (event) => {
        event.preventDefault();
        console.log('handling submit');
        // const data = {
        //     email: email,
        //     password: password
        // };
        console.log(event.target.id);
        const data = {
            email: event.target.email.value,
            password: event.target.password.value
        };
        const url = event.target.id == 'signup' ? 'signup' : 'login';

        let results = fetch(`http://localhost:8080/signup`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            credentials: "include",
            body: JSON.stringify(data)
        })
            .then(response => response.json())
            .then(data => {
                console.log(data);
                // setLoggedIn(true);
            })
            .catch(error => {
                console.error(error);
            });
    }


    return (
        <>
            <form onSubmit={(event) => {
                handleSubmit(event);
            }}>
                <div>
                    <label>Email:</label>
                    <input type="email" name="email" />
                </div>
                <div>
                    <label>Password:</label>
                    <input type="password" name="password" />
                </div>
                <div>
                    <button id='signup' type="submit" className='border bg-gray-400 rounded-md px-4 py-2 m-2'>signup</button>
                </div>
            </form>
            <form onSubmit={(event) => login(event)}>
                <div>
                    <label>Email:</label>
                    <input type="email" name="email" />
                </div>
                <div>
                    <label>Password:</label>
                    <input type="password" name="password" />
                </div>
                <div>
                    <button id='login' type="submit" className='border bg-gray-400 rounded-md px-4 py-2 m-2'>Login</button>
                </div>
            </form>
        </>

    );
}

