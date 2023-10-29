import { Input } from "./components/ui/input";
import { Button } from "./components/ui/button";
import { Label } from "./components/ui/label";
import { BASE_URL, SESSION_TOKEN_KEY } from "./lib/constants";
import { useContext, useState } from "react";
import { log } from "console";
import { redirect } from "@tanstack/react-router";
import { indexRoute } from "./main";
import { GlobalStateContext } from "./App";

/**
* v0 by Vercel.
* @see https://v0.dev/t/XNlTLb7
*/

export function Login() {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [loggedIn, setLoggedIn] = useState(false);

    let globalState = useContext(GlobalStateContext);

    const onSubmit = async (event) => {
        event.preventDefault();
        const loginPayload = JSON.stringify({ email: username, password: password });
        console.log('login component')
        console.log(loginPayload);
        console.log(globalState)
        try {
            const response = await fetch(`${BASE_URL}/auth/login`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: 'include',
                body: loginPayload,
            });

            if (response.status === 200) {
                const result = await response.json();
                const headers = await response.headers;
                const session_header = headers.get(SESSION_TOKEN_KEY);

                console.log('headers: ', headers)
                console.log('session header: ', session_header)
                setLoggedIn(true);
                globalState.setToken(session_header);
                console.log("Success:", result, headers);
                console.log('global state after login: ', JSON.stringify(globalState))
                // setToken(response.headers.get(SESSION_TOKEN_KEY));
            }

            redirect({
                to: "/", replace: true
            });

        } catch (error) {
            console.error("Error:", error);
        }
    };

    return (
        <>
            <input onChange={e => console.log('checking global state: ', JSON.stringify(globalState))}></input>
            {!loggedIn &&
                <div className="min-h-screen flex items-center justify-center" >
                    <div className="max-w-sm rounded-lg shadow-lg bg-white p-6 space-y-6 border border-gray-200 dark:border-gray-700" >
                        <h1 className="text-3xl font-bold space-y-2" > Login </h1>
                        < div className="space-y-4 text-left" >
                            <form onSubmit={onSubmit}>
                                <Label className="" htmlFor="email" > Email </Label>
                                <Input id="email" placeholder="m@example.com" required type="email" onChange={e => setUsername(e.target.value)} />
                                <Label className="" htmlFor="password" > Password </Label>
                                <Input id="password" required type="password" onChange={e => setPassword(e.target.value)} />
                                <Button className="border shadow-md p-2 w-full hover:bg-slate-400" type="submit" >
                                    Login
                                </Button>
                            </form>
                        </div>
                    </div>
                </div >
            }
        </>
    )
} 
