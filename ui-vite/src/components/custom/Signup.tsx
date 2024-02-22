import { Input } from "../ui/input";
import { Button } from "../ui/button";
import { Label } from "../ui/label";
import { SESSION_TOKEN_KEY } from "../../lib/constants";
import { FormEvent, useContext, useState } from "react";
import { getBaseUrl, handleResponse } from "../../lib/utils";
import { Link } from "react-router-dom";
// import { GlobalState, GlobalStateContext } from "./main";

/**
* v0 by Vercel.
* @see https://v0.dev/t/XNlTLb7
*/

export function Signup() {
    const [name, setName] = useState('');
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [loggedIn, setLoggedIn] = useState(false);

    // let globalState: GlobalState = useContext(GlobalStateContext);

    const onSubmit = async (event: FormEvent) => {
        event.preventDefault();
        const payload = JSON.stringify({ name: name, email: username, password });
        try {
            const response = await fetch(`${getBaseUrl()}/auth/signup`, {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: payload,
            });
            handleResponse(response)

            if (response.status === 200) {
                const result = await response.json();
                const session_header = response.headers.get(SESSION_TOKEN_KEY) || '';

                setLoggedIn(true);
                window.sessionStorage.setItem(SESSION_TOKEN_KEY, session_header);
            }

            // redirect({
            //     to: "/editor", replace: true
            // });

        } catch (error) {
            console.error("Error:", error);
        }
    };

    return (
        <>
            {!loggedIn &&
                <div className="min-h-screen flex items-center justify-center w-240" >
                    <div className="max-w-sm rounded-lg shadow-lg bg-white p-6 space-y-6 border border-gray-200 dark:border-gray-700" >
                        <h1 className="text-3xl font-bold space-y-2" >
                            Signup
                        </h1>
                        < div className="space-y-4 text-left" >
                            <form onSubmit={onSubmit}>
                                <Label className="" htmlFor="name" >Name</Label>
                                <Input id="name" placeholder="" required type="text" onChange={e => setName(e.target.value)} />
                                <Label className="" htmlFor="email" > Email </Label>
                                <Input id="email" placeholder="m@example.com" required type="email" onChange={e => setUsername(e.target.value)} />
                                <Label className="" htmlFor="password" > Password </Label>
                                <Input id="password" required type="password" onChange={e => setPassword(e.target.value)} />
                                <div className="p-4"></div>
                                <Button className="border shadow-md p-2 w-full hover:bg-slate-400" type="submit" >
                                    Signup
                                </Button>
                                <div className="text-center p-1">{"Already have an account? "}
                                    <Link className="underline" to="/login">
                                        Login here
                                    </Link>
                                </div>
                            </form>
                        </div>
                    </div>
                </div >
            }
            {loggedIn &&
                <div className="pt-10">
                    Please check your email and click the confirmation link to proceed
                </div>
            }
        </>
    )
} 
