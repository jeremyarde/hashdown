import React, { StrictMode } from 'react'
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

// Create a root route
const rootRoute = new RootRoute({
  component: App,
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
  component: Editor,
})

const renderSurveyRoute = new Route({
  getParentRoute: () => surveysRoute,
  path: '$surveyId',
  component: ListSurveys,
})


// Create the route tree using your routes
const routeTree = rootRoute.addChildren([
  // indexRoute, 
  editorRoute,
  loginRoute,
  surveysRoute.addChildren([renderSurveyRoute])
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
