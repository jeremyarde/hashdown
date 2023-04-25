import React, { useState } from 'react';
// import { nanoid_gen } from "../../../backend/pkg";
// import { CreateSurveyRequest } from "../../../server/bindings/CreateSurveyRequest";
// import { supabase } from "../supabase";

export default function Login() {
    // const [email, setEmail] = useState('');
    // const [password, setPassword] = useState('');
    // const [loggedin, setLoggedIn] = useState(false);

    // const handleEmailChange = (event) => {
    //     setEmail(event.target.value);
    // }

    // const handlePasswordChange = (event) => {
    //     setPassword(event.target.value);
    // }

    const handleSubmit = (event) => {
        event.preventDefault();
        console.log('handling submit');
        // const data = {
        //     email: email,
        //     password: password
        // };
        const data = {
            email: event.target.email.value,
            password: event.target.password.value
        };
        let results = fetch('http://localhost:8080/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
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
        <form onSubmit={handleSubmit}>
            <div>
                <label>Email:</label>
                <input type="email" name="email" />
            </div>
            <div>
                <label>Password:</label>
                <input type="password" name="password" />
            </div>
            <div>
                <button type="submit">Login</button>
            </div>
        </form>
    );
}

