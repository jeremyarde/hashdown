/**
 * v0 by Vercel.
 * @see https://v0.dev/t/LUoP6hiokbX
 */
// import Link from "next/link"
import { Button } from "@/components/ui/button"
import { SelectTrigger, SelectItem, SelectGroup, SelectContent, Select } from "./components/ui/select"
import { Link } from "@tanstack/react-router"

export function Navbar() {
    return (
        <>
            <div className="flex items-center justify-between p-2 w-full shadow-md">
                <div>
                    <Link className="text-2xl font-bold" to="/">
                        <span>Form MD</span>
                    </Link>
                </div>
                <div className="flex items-center space-x-4">
                    <Link to="/login">Login</Link>
                    {/* <Button variant="outline"><Link to="/editor">Editor</Link></Button> */}
                    <Link to="/surveys">Surveys</Link>
                    <Link to="/editor">Editor</Link>
                    <Select>
                        <SelectTrigger>
                            <Button variant="outline">Menu</Button>
                        </SelectTrigger>
                        <SelectContent className="mt-2">
                            <SelectGroup>
                                <SelectItem value="My Surveys">My Surveys</SelectItem>
                                <SelectItem value="option2">Option 2</SelectItem>
                                <SelectItem value="option3">Option 3</SelectItem>
                            </SelectGroup>
                        </SelectContent>
                    </Select>
                </div>
            </div >
        </>
    )
}
