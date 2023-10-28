/**
 * v0 by Vercel.
 * @see https://v0.dev/t/LUoP6hiokbX
 */
// import Link from "next/link"
import { Button } from "@/components/ui/button"
import { SelectTrigger, SelectItem, SelectGroup, SelectContent, Select } from "./components/ui/select"

export function Navbar() {
    return (
        <div className="flex items-center justify-between p-6">
            <div>
                <a className="text-2xl font-bold" href="#">
                    <span>BrandName</span>
                </a>
            </div>
            <div className="flex items-center space-x-4">
                <Button variant="outline">Login</Button>
                <Select>
                    <SelectTrigger>
                        <Button variant="outline">Menu</Button>
                    </SelectTrigger>
                    <SelectContent className="mt-2">
                        <SelectGroup>
                            <SelectItem value="option1">Option 1</SelectItem>
                            <SelectItem value="option2">Option 2</SelectItem>
                            <SelectItem value="option3">Option 3</SelectItem>
                        </SelectGroup>
                    </SelectContent>
                </Select>
            </div>
        </div>
    )
}
