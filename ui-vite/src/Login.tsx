import { Input } from "./components/ui/input";
import { Button } from "./components/ui/button";
import { Label } from "./components/ui/label";
import { BASE_URL, SESSION_TOKEN_KEY } from "./lib/constants";
import { useContext, useState } from "react";
import { GlobalState, GlobalStateContext } from "./main";
import { Navigate, redirect } from "react-router-dom";
import { handleResponse, setSessionToken } from "./lib/utils";

/**
* v0 by Vercel.
* @see https://v0.dev/t/XNlTLb7
*/

enum ComponentState {
    Loading,
    Error,
    Idle,
}

export function Login() {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [loggedIn, setLoggedIn] = useState(false);
    const [loginError, setLoginError] = useState('');
    const [isLoading, setIsLoading] = useState(false);
    const [componentState, setComponentState] = useState(ComponentState.Idle);


    let globalState: GlobalState = useContext(GlobalStateContext);

    const onSubmit = async (event) => {
        event.preventDefault();
        setIsLoading(true);
        const loginPayload = JSON.stringify({ email: username, password: password });
        console.log('login component')
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
            const result = await response.json();
            handleResponse(response, globalState);

            if (response.status === 200) {
                // const result = await response.json();
                setLoggedIn(true);
                setSessionToken(response, globalState);
                setLoginError('');
            } else if (response.status === 401) {
                setLoginError(result.error.message);
                setLoggedIn(false);
                setComponentState(ComponentState.Error);
            } else {
                setLoggedIn(false);
                setLoginError(result.error);
            }
        } catch (error) {
            console.error("Error:", error);

        }
    };

    let componentHtml = undefined;
    switch (componentState) {
        case ComponentState.Error:
            componentHtml = (<div>{`${loginError}`}</div>)
            break;
        case ComponentState.Loading:
            componentHtml = (<div className="bg-gray-200 w-full flex justify-center items-center">
                <div className="flex  w-full items-center justify-center bg-gray-200">
                    <div className="flex h-14 w-14 items-center justify-center rounded-full bg-gradient-to-tr from-indigo-500 to-pink-500 animate-spin">
                        <div className="h-9 w-9 rounded-full bg-gray-600"></div>
                    </div>
                </div>
            </div>)
            break;
        case ComponentState.Idle:
            componentHtml = '';
            break;
    }

    return (
        <>
            {loggedIn && <Navigate to={"/editor"} />}
            {!loggedIn &&
                <div className="min-h-screen flex items-center justify-center w-240" >
                    <div className="max-w-sm rounded-lg shadow-lg bg-white p-6 space-y-6 border border-gray-200 dark:border-gray-700" >
                        <h1 className="text-3xl font-bold space-y-2" >
                            Login
                        </h1>
                        <div className="space-y-4 text-left" >
                            <form onSubmit={onSubmit}>
                                <Label className="" htmlFor="email" > Email </Label>
                                <Input id="email" placeholder="m@example.com" required type="email" onChange={e => setUsername(e.target.value)}
                                    className="invalid:border-pink-500 invalid:text-pink-600" />
                                <Label className="" htmlFor="password" > Password </Label>
                                <Input id="password" required type="password" onChange={e => setPassword(e.target.value)} />
                                <div className="p-4">
                                    {componentHtml ? (componentHtml) : (<div></div>)}
                                </div>
                                <Button disabled={isLoading} className="border shadow-md p-2 w-full hover:bg-slate-400 hover:bg-green disabled:opacity-50" type="submit" >
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
