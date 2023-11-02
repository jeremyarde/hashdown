import { createContext, useState } from 'react'
import './App.css'



import { markdown_to_form_wasm } from '../../backend/pkg/markdownparser';
import { Textarea } from './components/ui/textarea';
import { Editor } from './pages/Editor';
import { ListSurveys } from './pages/ListSurveys';
import { RenderedForm } from './RenderedForm';
import { Navbar } from './Navbar';
import { Login } from './Login';
import { Outlet } from '@tanstack/react-router';


export type GlobalState = {
  token: string;
  setToken: React.Dispatch<React.SetStateAction<string>>,
}
export const GlobalStateContext = createContext();


export function App() {
  const [formtext, setFormtext] = useState('# A survey title here\n- q1\n  - option 1\n  - option 2\n  - option 3\n- question 2\n  - q2 option 1\n  - q2 option 2"');
  const survey = markdown_to_form_wasm(formtext);
  const [token, setToken] = useState(window.sessionStorage.getItem('session_id') ?? '');

  let globalState: GlobalState = {
    token: token,
    setToken: setToken,
  }

  return (
    <>
      <GlobalStateContext.Provider value={globalState}>
        <Navbar ></Navbar>
        <Outlet />
      </GlobalStateContext.Provider >
    </>
  )
}
