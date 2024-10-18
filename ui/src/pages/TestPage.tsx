/**
 * v0 by Vercel.
 * @see https://v0.dev/t/RyBXMVzZygA
 */
import { AvatarImage, Avatar } from "@/components/ui/avatar"
import { CardTitle, CardHeader, CardContent, Card } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { SelectValue, SelectTrigger, SelectItem, SelectContent, Select } from "@/components/ui/select"
import { Input } from "@/components/ui/input"
import { TableHead, TableRow, TableHeader, TableCell, TableBody, Table } from "@/components/ui/table"

export default function Component() {
    return (
        <div className="bg-[#f4f7fa] min-h-screen">
            <div className="grid grid-cols-12 gap-4 p-8">
                <aside className="col-span-2 bg-white rounded-lg p-4">
                    <div className="flex items-center space-x-2 mb-6">
                        <BanknoteIcon className="h-8 w-8" />
                        <span className="font-bold">FinBank</span>
                    </div>
                    <nav className="space-y-2">
                        <a className="block p-2 rounded-lg bg-[#e8f0fe] text-[#3b82f6]" href="#">
                            Dashboard
                        </a>
                        <a className="block p-2 rounded-lg hover:bg-gray-100" href="#">
                            Transactions
                        </a>
                        <a className="block p-2 rounded-lg hover:bg-gray-100" href="#">
                            Card Center
                        </a>
                        <a className="block p-2 rounded-lg hover:bg-gray-100" href="#">
                            Contacts
                        </a>
                        <a className="block p-2 rounded-lg hover:bg-gray-100" href="#">
                            E-Wallet Center
                        </a>
                        <a className="block p-2 rounded-lg hover:bg-gray-100" href="#">
                            Reports
                        </a>
                        <a className="block p-2 rounded-lg hover:bg-gray-100" href="#">
                            Settings
                        </a>
                        <a className="block p-2 rounded-lg hover:bg-gray-100" href="#">
                            Help Center
                        </a>
                    </nav>
                    <div className="text-xs text-gray-500 mt-6">
                        © FinBank. 2020
                        <br />
                        Digital Payment Platform for solutions of all types of business.
                    </div>
                </aside>
                <main className="col-span-10 space-y-4">
                    <header className="flex justify-between items-center">
                        <h1 className="text-xl font-semibold">Welcome to FinBank</h1>
                        <div className="flex items-center space-x-4">
                            <PhoneIcon className="h-6 w-6" />
                            <MessageCircleIcon className="h-6 w-6" />
                            <Avatar>
                                <AvatarImage alt="User profile" src="/placeholder.svg?height=32&width=32" />
                            </Avatar>
                        </div>
                    </header>
                    <section className="grid grid-cols-3 gap-4">
                        <Card className="col-span-1 bg-[#e8f0fe]">
                            <CardHeader>
                                <CardTitle>Debit Card Account</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <div className="flex items-center justify-between">
                                    <div>
                                        <CreditCardIcon className="h-12 w-12 text-[#3b82f6]" />
                                        <p className="text-sm">4771 6080 1080 7889</p>
                                        <p className="text-xs">Valid Thru 08/25</p>
                                    </div>
                                    <Button variant="ghost">Add Debit Card</Button>
                                </div>
                            </CardContent>
                        </Card>
                        <Card className="col-span-2 bg-white">
                            <CardHeader>
                                <CardTitle>Your Total Balance</CardTitle>
                            </CardHeader>
                            <CardContent>
                                <div className="flex justify-between items-center">
                                    <div>
                                        <p className="text-3xl font-semibold">$ 80,201.50</p>
                                        <p className="text-sm">December 21, 2020 - 02:20 PM</p>
                                    </div>
                                    <div className="grid grid-cols-3 gap-2">
                                        <Button variant="ghost">Send</Button>
                                        <Button variant="ghost">Topup</Button>
                                        <Button variant="ghost">More</Button>
                                    </div>
                                </div>
                            </CardContent>
                        </Card>
                    </section>
                    <section className="grid grid-cols-3 gap-4">
                        <div className="col-span-2">
                            <Card className="bg-white">
                                <CardHeader>
                                    <CardTitle>Recent Transactions</CardTitle>
                                    <Select>
                                        <SelectTrigger id="timeframe">
                                            <SelectValue placeholder="Last 7 Days" />
                                        </SelectTrigger>
                                        <SelectContent position="popper">
                                            <SelectItem value="last-7-days">Last 7 Days</SelectItem>
                                            <SelectItem value="last-30-days">Last 30 Days</SelectItem>
                                            <SelectItem value="last-90-days">Last 90 Days</SelectItem>
                                        </SelectContent>
                                    </Select>
                                </CardHeader>
                                <CardContent>
                                    <div className="space-y-2">
                                        <div className="flex justify-between items-center">
                                            <div className="flex items-center space-x-2">
                                                <CreditCardIcon className="h-6 w-6" />
                                                <div>
                                                    <p className="font-medium">Paypal - Received</p>
                                                    <p className="text-sm text-gray-500">20 December 2020, 08:20 AM</p>
                                                </div>
                                            </div>
                                            <p className="text-green-500">+ $8,200.00</p>
                                        </div>
                                        <div className="flex justify-between items-center">
                                            <div className="flex items-center space-x-2">
                                                <PodcastIcon className="h-6 w-6" />
                                                <div>
                                                    <p className="font-medium">Spotify Premium</p>
                                                    <p className="text-sm text-gray-500">19 December 2020, 07:25 PM</p>
                                                </div>
                                            </div>
                                            <p className="text-red-500">- $199.00</p>
                                        </div>
                                        <div className="flex justify-between items-center">
                                            <div className="flex items-center space-x-2">
                                                <WalletIcon className="h-6 w-6" />
                                                <div>
                                                    <p className="font-medium">Transferwise - Received</p>
                                                    <p className="text-sm text-gray-500">19 December 2020, 10:15 AM</p>
                                                </div>
                                            </div>
                                            <p className="text-green-500">+ $1,200.00</p>
                                        </div>
                                        <div className="flex justify-between items-center">
                                            <div className="flex items-center space-x-2">
                                                <MSquareIcon className="h-6 w-6" />
                                                <div>
                                                    <p className="font-medium">H&M Payment</p>
                                                    <p className="text-sm text-gray-500">15 December 2020, 06:30 PM</p>
                                                </div>
                                            </div>
                                            <p className="text-red-500">- $2,200.00</p>
                                        </div>
                                    </div>
                                </CardContent>
                            </Card>
                        </div>
                        <div className="col-span-1">
                            <Card className="bg-white">
                                <CardHeader>
                                    <CardTitle>Expenses Instead</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    {/* <PieChart className="w-full h-[200px]" /> */}
                                    <div className="text-center">
                                        <p className="text-4xl font-semibold">85.5%</p>
                                        <p className="text-sm">Normal Level</p>
                                        <p className="text-sm">Total Exp: $1,820.80</p>
                                    </div>
                                </CardContent>
                            </Card>
                        </div>
                    </section>
                    <section className="bg-white rounded-lg p-4">
                        <div className="flex justify-between items-center mb-4">
                            <h2 className="font-semibold">Invoice Activity</h2>
                            <div className="flex items-center space-x-2">
                                <Input placeholder="Search..." type="search" />
                                <Button variant="outline">Filter</Button>
                                <Button variant="outline">Export</Button>
                            </div>
                        </div>
                        <Table>
                            <TableHeader>
                                <TableRow>
                                    <TableHead className="w-[150px]">DATE & TIME</TableHead>
                                    <TableHead className="w-[150px]">INVOICE NUMBER</TableHead>
                                    <TableHead>RECIPIENT</TableHead>
                                    <TableHead>STATUS</TableHead>
                                    <TableHead>ACTION</TableHead>
                                    <TableHead className="w-[100px]">AMOUNT</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                <TableRow>
                                    <TableCell>21 Dec 2020, 02:20 PM</TableCell>
                                    <TableCell>INV001</TableCell>
                                    <TableCell>John Doe</TableCell>
                                    <TableCell>Paid</TableCell>
                                    <TableCell>
                                        <Button variant="ghost">View</Button>
                                    </TableCell>
                                    <TableCell>$500.00</TableCell>
                                </TableRow>
                            </TableBody>
                        </Table>
                    </section>
                </main>
            </div>
        </div>
    )
}

function BanknoteIcon(props: any) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <rect width="20" height="12" x="2" y="6" rx="2" />
            <circle cx="12" cy="12" r="2" />
            <path d="M6 12h.01M18 12h.01" />
        </svg>
    )
}


function CreditCardIcon(props: any) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <rect width="20" height="14" x="2" y="5" rx="2" />
            <line x1="2" x2="22" y1="10" y2="10" />
        </svg>
    )
}


function MSquareIcon(props: any) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <rect width="18" height="18" x="3" y="3" rx="2" />
            <path d="M8 16V8l4 4 4-4v8" />
        </svg>
    )
}


function MessageCircleIcon(props: any) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <path d="m3 21 1.9-5.7a8.5 8.5 0 1 1 3.8 3.8z" />
        </svg>
    )
}


function PhoneIcon(props: any) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z" />
        </svg>
    )
}


function PodcastIcon(props: any) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <circle cx="12" cy="11" r="1" />
            <path d="M11 17a1 1 0 0 1 2 0c0 .5-.34 3-.5 4.5a.5.5 0 0 1-1 0c-.16-1.5-.5-4-.5-4.5Z" />
            <path d="M8 14a5 5 0 1 1 8 0" />
            <path d="M17 18.5a9 9 0 1 0-10 0" />
        </svg>
    )
}


function WalletIcon(props: any) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <path d="M21 12V7H5a2 2 0 0 1 0-4h14v4" />
            <path d="M3 5v14a2 2 0 0 0 2 2h16v-5" />
            <path d="M18 12a2 2 0 0 0 0 4h4v-4Z" />
        </svg>
    )
}
