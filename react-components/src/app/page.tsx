'use client';

import Image from 'next/image'
import Login from '../components/Login'
// import Survey from './Survey'
import App from '../components/App';


export default function Home() {
  return (
    <main>
      <div className='grid grid-cols-3'>
        <App></App>
      </div>
    </main>)
  // <main className="flex min-h-screen flex-col items-center justify-between p-24 bg-yellow-200 ">
  {/* <NavigationMenu>
        <NavigationMenuList>
          <NavigationMenuItem>
            <NavigationMenuTrigger>Item One</NavigationMenuTrigger>
            <NavigationMenuContent>
              <NavigationMenuLink>Link</NavigationMenuLink>
            </NavigationMenuContent>
          </NavigationMenuItem>
        </NavigationMenuList>
      </NavigationMenu> */}
  // <App></App>
  // </main >
  // )
}
