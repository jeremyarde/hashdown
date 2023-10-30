import React, { StrictMode, useState } from 'react'
import ReactDOM from 'react-dom/client'
import { App } from './App.tsx'
import './index.css'

import {
  Outlet,
  RouterProvider,
  Link,
  Router,
  Route,
  RootRoute,
} from '@tanstack/react-router'
import { Login } from './Login.tsx'
import { Navbar } from './Navbar.tsx'
import { ListSurveys } from './pages/ListSurveys.tsx'
import { Editor } from './pages/Editor.tsx'

import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { RenderedForm } from './RenderedForm.tsx'
import { markdown_to_form_wasm } from '../../backend/pkg/markdownparser'

// Create a root route
// const rootRoute = new RootRoute({
//   component: App,
// })
const rootRoute = new RootRoute({
  // component: () => {
  //   return (
  //     <>
  //       <div className="p-2 flex gap-2 text-lg">
  //         <Link
  //           to="/"
  //           activeProps={{
  //             className: 'font-bold',
  //           }}
  //           activeOptions={{ exact: true }}
  //         >
  //           Home
  //         </Link>{' '}
  //         <Link
  //           to={'/login'}
  //           activeProps={{
  //             className: 'font-bold',
  //           }}
  //         >
  //           Posts
  //         </Link>
  //       </div>
  //       <hr />
  //       <Outlet />
  //       {/* Start rendering router matches */}
  //       <TanStackRouterDevtools position="bottom-right" />
  //     </>
  //   )
  // },
  component: App
})

const indexRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/',
  component: () => {
    return (
      <div className="p-2">
        <h3>Welcome Home!</h3>
      </div>
    )
  },
})

const loginRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/login',
  component: Login,
})

const surveysRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/surveys',
  component: ListSurveys,
})

const editorRoute = new Route({
  getParentRoute: () => rootRoute,
  path: '/editor',
  component: () => {
    const [formtext, setFormtext] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"');
    const survey = markdown_to_form_wasm(formtext);
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
  // surveysRoute.addChildren([renderSurveyRoute])
  surveysRoute
]);

// Create the router using your route tree
const router = new Router({ routeTree })

// Register your router for maximum type safety
declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

// ReactDOM.createRoot(document.getElementById('root')!).render(
//   <React.StrictMode>
//     <App />
//   </React.StrictMode>,
// )

// Render our app!
const rootElement = document.getElementById('root')!

if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement)
  root.render(
    <StrictMode>
      <RouterProvider router={router} />
    </StrictMode>,
  )
}
