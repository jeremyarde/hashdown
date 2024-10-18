/**
 * v0 by Vercel.
 * @see https://v0.dev/t/zIExRf9XQ9l
 */
// import a from "next/link"
import { Input } from "@/components/ui/input"
import { CardTitle, CardHeader, CardContent, Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { TITLE } from "@/lib/config"

export default function Component() {
    return (
        <div className="flex h-screen bg-gray-100 dark:bg-gray-900">
            <div className="flex flex-col w-64 bg-white dark:bg-gray-800 px-8 py-4">
                <h2 className="text-2xl font-semibold text-gray-800 dark:text-white">{TITLE}</h2>
                <nav className="flex-grow mt-6">
                    <a
                        className="block text-gray-800 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 px-4 py-2 rounded"
                        href="#"
                    >
                        Create New Form
                    </a>
                    <a
                        className="block text-gray-800 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 px-4 py-2 rounded mt-2"
                        href="#"
                    >
                        Manage Forms
                    </a>
                    <a
                        className="block text-gray-800 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 px-4 py-2 rounded mt-2"
                        href="#"
                    >
                        User Settings
                    </a>
                </nav>
            </div>
            <div className="flex flex-col flex-grow overflow-hidden">
                <header className="flex items-center justify-between px-10 py-4 border-b">
                    <h2 className="text-lg font-semibold text-gray-800 dark:text-white">Dashboard</h2>
                    <div className="relative">
                        <span className="absolute inset-y-0 left-0 flex items-center pl-3">
                            {/* <SearchIcon className="w-5 h-5 text-gray-500 dark:text-gray-400" /> */}
                        </span>
                        <Input
                            className="pl-10 pr-4 py-2 rounded-lg bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-gray-300"
                            placeholder="Search forms..."
                            type="search"
                        />
                    </div>
                </header>
                <main className="flex-grow p-10 overflow-y-auto">
                    <Card>
                        <CardHeader className="flex justify-between items-center">
                            <CardTitle className="text-lg font-semibold text-gray-800 dark:text-white">Form 1</CardTitle>
                            <Button>Edit</Button>
                        </CardHeader>
                        <CardContent>
                            <p className="text-gray-600 dark:text-gray-400">Submissions: 120</p>
                            <p className="text-gray-600 dark:text-gray-400">Created: January 1, 2024</p>
                        </CardContent>
                    </Card>
                    <Card className="mt-4">
                        <CardHeader className="flex justify-between items-center">
                            <CardTitle className="text-lg font-semibold text-gray-800 dark:text-white">Form 2</CardTitle>
                            <Button>Edit</Button>
                        </CardHeader>
                        <CardContent>
                            <p className="text-gray-600 dark:text-gray-400">Submissions: 85</p>
                            <p className="text-gray-600 dark:text-gray-400">Created: February 10, 2024</p>
                        </CardContent>
                    </Card>
                </main>
            </div>
        </div>
    )
}
