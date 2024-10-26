import { useState } from "react";
import "./App.css";
import { AppTable } from "./components/app-table";
import { AppEditor } from "./components/app-editor";

import { Switch, Route } from "wouter";
import MainPage from "./pages/main-page";

function App() {
  return (
    <>
      {/* <Layout> */}
      <Switch>
        <Route path="/" component={MainPage} />
        <Route path="/editor" component={AppEditor} />
        <Route path="/forms" component={AppTable} />
        <Route path="/components" component={AppTable} />
        <Route>404, Not Found!</Route>
      </Switch>
      {/* </Layout> */}
      {/* <Layout>
        <Button>Click me</Button>
        <AppEditor />
        <AppTable />
      </Layout> */}
    </>
  );
}

export default App;
