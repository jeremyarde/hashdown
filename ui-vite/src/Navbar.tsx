/**
 * v0 by Vercel.
 * @see https://v0.dev/t/LUoP6hiokbX
 */
// import Link from "next/link"
import { Button } from "@/components/ui/button"
import { SelectTrigger, SelectItem, SelectGroup, SelectContent, Select } from "./components/ui/select"
import { Link } from "@tanstack/react-router"
import { useContext } from "react";
import { GlobalState, GlobalStateContext } from "./main";
import { BASE_URL, SESSION_TOKEN_KEY } from "./lib/constants";


export function Navbar() {
    let globalState: GlobalState = useContext(GlobalStateContext);

    console.log(`current path: ${'test'}`)

    async function logout() {
        console.log('logging out');
        // const response = await fetch(`${BASE_URL}/auth/logout`, {
        //     method: "POST",
        //     headers: {
        //         "Content-Type": "application/json",
        //     },
        //     // credentials: 'include',
        //     // body: payload,
        // });
        window.sessionStorage.removeItem(SESSION_TOKEN_KEY);
        globalState.setToken(undefined);
    };

    return (
        <>
            <div className="flex items-center justify-between w-full">
                <div>
                    <Link className="text-2xl font-bold" to="/">
                        <span>Form MD</span>
                    </Link>
                </div>
                <div className="flex items-center space-x-4">
                    {!globalState.token ? (
                        <>
                            <Link className="hover:animate-pulse" to="/editor">Editor</Link>
                            <Link className="hover:animate-pulse" to="/login">Login</Link>
                        </>
                    ) : (
                        <>
                            <Link className="hover:animate-pulse" to="/surveys">Surveys</Link>
                            <Link className="hover:animate-pulse" to="/editor">Editor</Link>
                            <Link className="hover:animate-pulse" onClick={(e) => logout()} > Logout</Link>
                        </>
                    )
                    }
                </div>
            </div >
        </>
    )
}
