'use client';

import Image from 'next/image'
import Login from './Login'
import Survey from './Survey'
import App from './App';

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10 w-full max-w-5xl items-center justify-between font-mono text-sm lg:flex text-blue-600">
        {/* <Login></Login>
        <Survey></Survey> */}
        <App></App>
      </div>
    </main>
  )
}
