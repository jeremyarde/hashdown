/**
 * v0 by Vercel.
 * @see https://v0.dev/t/LUoP6hiokbX
 */
// import Link from "next/link"
import { Button } from "@/components/ui/button"
import { SelectTrigger, SelectItem, SelectGroup, SelectContent, Select } from "./components/ui/select"
import { Link } from "@tanstack/react-router"
import { useContext } from "react";
import { GlobalState, GlobalStateContext } from "./App";

export function Navbar() {
    let globalState: GlobalState = useContext(GlobalStateContext);


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
                            <Link className="hover:animate-pulse" onClick={(e) => console.log('logout')}>Logout</Link>
                        </>
                    )
                    }
                </div>
            </div >
        </>
    )
}
