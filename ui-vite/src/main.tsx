import React, { StrictMode, createContext, useEffect, useState } from 'react'
import ReactDOM from 'react-dom/client'
import './index.css'

import {
  Outlet,
  RouterProvider,
  Link,
  Router,
  Route,
  RootRoute,
  redirect,
  RouterContext,
} from '@tanstack/react-router'
import { Login } from './Login.tsx'
import { Navbar } from './Navbar.tsx'
import { ListSurveys } from './pages/ListSurveys.tsx'
import { Editor } from './pages/Editor.tsx'

import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { RenderedForm } from './RenderedForm.tsx'
import { markdown_to_form_wasm } from '../../backend/pkg/markdownparser'
import { Signup } from './Signup.tsx'
import { Button } from './components/ui/button.tsx'

// Create a root route
// const rootRoute = new RootRoute({
//   component: App,
// })
// const TanStackRouterDevtools =
//   process.env.NODE_ENV === 'production'
//     ? () => null // Render nothing in production
//     : React.lazy(() =>
//       // Lazy load in development
//       import('@tanstack/router-devtools').then((res) => ({
//         default: res.TanStackRouterDevtools,
//         // For Embedded Mode
//         // default: res.TanStackRouterDevtoolsPanel
//       })),
//     );


export type GlobalState = {
  token: string;
  setToken: React.Dispatch<React.SetStateAction<string>>,
}
// export const GlobalStateContext = createContext();

const routerContext = new RouterContext<GlobalState>()


function App() {
  const [formtext, setFormtext] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"');
  const survey = markdown_to_form_wasm(formtext);

  return (
    <>
      <Navbar></Navbar>
    </>
  )
}



const rootRoute = routerContext.createRootRoute({
  component: () => {
    const [token, setToken] = useState(window.sessionStorage.getItem('session_id') ?? '');

    // let globalState: GlobalState = {
    //   token: token,
    //   setToken: setToken,
    // }
    return (
      <>
        {/* <GlobalStateContext.Provider value={globalState}> */}
        <App />
        {/* </GlobalStateContext.Provider > */}
        <Outlet />
        <TanStackRouterDevtools />
      </>
    )
  },
});

const indexRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/',
  component: () => {
    // redirect({
    //   to: "/editor", replace: true
    // });
    return (
      <>
        <h1 className='flex top-10 text-center justify-center m-12'>The easiest way to create and share surveys</h1>
        <Link to='/editor'>
          <Button className='bg-blue-400 rounded-lg'>
            Get started
          </Button>
        </Link>
      </>
    )
  },
})

const loginRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/login',
  component: Login,
})

const signupRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/signup',
  component: Signup,
})

const surveysRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/surveys',
  component: ListSurveys,
})

const surveyRoute = new Route({
  getParentRoute: () => surveysRoute,
  path: '/$surveyId',
  component: (params) => {
    return (<>
      <div>Show me the survey</div>
      <div>{JSON.stringify(params)}</div>
    </>)
  },
})

const editorRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/editor',
  component: () => {
    const [formtext, setFormtext] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"');
    const [survey, setSurvey] = useState(undefined);

    useEffect(() => {
      console.log('editor useeffect');
      const newSurvey = markdown_to_form_wasm(formtext);
      setSurvey(newSurvey);
    }, [formtext]);
    // const [token, setToken] = useState('');

    return (
      <div className="h-screen w-full flex">
        <div className="w-1/2 border-r-2 p-4">
          <Editor editorContent={formtext} setEditorContent={setFormtext}></Editor>
        </div>
        <div className="w-1/2 p-4">
          <h1 className="text-2xl font-bold mb-4">Preview</h1>
          <div className="border border-gray-300 p-4 rounded">
            <RenderedForm plaintext={formtext} survey={survey} ></RenderedForm>
          </div>
        </div>
      </div>)
  },
})

// const renderSurveyRoute = new Route({
//   getParentRoute: () => surveysRoute,
//   path: '$surveyId',
//   component: ListSurveys,
// })


// Create the route tree using your routes
const routeTree = rootRoute.addChildren([
  indexRoute,
  editorRoute,
  loginRoute,
  signupRoute,
  // surveysRoute.addChildren([renderSurveyRoute])
  surveysRoute.addChildren([surveyRoute])
]);

// Create the router using your route tree
const router = new Router({
  routeTree, context: {
    token: ''
  }
})

// Register your router for maximum type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

// Render our app!

const rootElement = document.getElementById('root')!

if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement)
  root.render(
    <StrictMode>
      <RouterProvider router={router} />
    </StrictMode>
  )
}
