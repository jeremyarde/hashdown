import React, { useState } from 'react';
// import { nanoid_gen } from "../../../backend/pkg";
// import { CreateSurveyRequest } from "../../../server/bindings/CreateSurveyRequest";
// import { supabase } from "../supabase";

export default async function Login() {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [loggedin, setLoggedIn] = useState(false);

    const handleEmailChange = (event) => {
        setEmail(event.target.value);
    }

    const handlePasswordChange = (event) => {
        setPassword(event.target.value);
    }

    const handleSubmit = (event) => {
        event.preventDefault();

        const data = {
            email: email,
            password: password
        };

        let results = fetch('/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(data)
        })
            .then(response => response.json())
            .then(data => {
                console.log(data);
                setLoggedIn(true);
            })
            .catch(error => {
                console.error(error);
            });
    }

    return (
        <>
            loggedin ?  {"We are logged in already"} :
            <form onSubmit={handleSubmit}>
                <div>
                    <label>Email:</label>
                    <input type="email" value={email} onChange={handleEmailChange} />
                </div>
                <div>
                    <label>Password:</label>
                    <input type="password" value={password} onChange={handlePasswordChange} />
                </div>
                <div>
                    <button type="submit">Login</button>
                </div>
            </form>

        </>
    )
}

