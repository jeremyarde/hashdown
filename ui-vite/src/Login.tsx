import { Input } from "./components/ui/input";
import { Button } from "./components/ui/button";
import { Label } from "./components/ui/label";
import { BASE_URL, SESSION_TOKEN_KEY } from "./lib/constants";
import { useContext, useState } from "react";
import { GlobalState, GlobalStateContext } from "./main";
import { Navigate, redirect } from "react-router-dom";

/**
* v0 by Vercel.
* @see https://v0.dev/t/XNlTLb7
*/

export function Login() {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [loggedIn, setLoggedIn] = useState(false);
    const [loginError, setLoginError] = useState('');


    let globalState: GlobalState = useContext(GlobalStateContext);

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
                const session_header = response.headers.get(SESSION_TOKEN_KEY);

                setLoggedIn(true);
                globalState.setToken(session_header);
                window.sessionStorage.setItem(SESSION_TOKEN_KEY, session_header);
                // return redirect("/editor");
            } else {
                setLoginError((prev) => result.message);
            }
        } catch (error) {
            console.error("Error:", error);

        }
    };

    return (
        <>
            {loggedIn && <Navigate to={"/editor"} />}
            {!loggedIn &&
                <div className="min-h-screen flex items-center justify-center w-240" >
                    <div className="max-w-sm rounded-lg shadow-lg bg-white p-6 space-y-6 border border-gray-200 dark:border-gray-700" >
                        <h1 className="text-3xl font-bold space-y-2" >
                            Login
                        </h1>
                        < div className="space-y-4 text-left" >
                            <form onSubmit={onSubmit}>
                                <Label className="" htmlFor="email" > Email </Label>
                                <Input id="email" placeholder="m@example.com" required type="email" onChange={e => setUsername(e.target.value)}
                                    className="invalid:border-pink-500 invalid:text-pink-600" />
                                <Label className="" htmlFor="password" > Password </Label>
                                <Input id="password" required type="password" onChange={e => setPassword(e.target.value)} />
                                {loginError ? (<p>{`Login failed: ${loginError}`}</p>) : (<div className="p-4"></div>)}
                                <Button className="border shadow-md p-2 w-full hover:bg-slate-400" type="submit" >
                                    Login
                                </Button>
                                <div className="text-center p-1">{"No account? "}
                                    <a className="underline" href="/signup">
                                        Signup here
                                    </a>
                                </div>
                            </form>
                        </div>
                    </div>
                </div >
            }
        </>
    )
} 
