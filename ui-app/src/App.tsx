import { useState } from "react";
import "./App.css";
import { Button } from "./components/ui/button";
import Layout from "./app/layout";
import { AppTable } from "./components/app-table";
import { AppCrumbs } from "./components/app-crumbs";
import { AppEditor } from "./components/app-editor";

import { Switch, Route } from "wouter";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <Layout>
        <Switch>
          {/* <Route path="/" component={Layout} /> */}
          <Route path="/editor" component={AppEditor} />
          <Route path="/forms" component={AppTable} />
          <Route path="/components" component={AppTable} />
          <Route>404, Not Found!</Route>
        </Switch>
      </Layout>
      {/* <Layout>
        <Button>Click me</Button>
        <AppEditor />
        <AppTable />
      </Layout> */}
    </>
  );
}

export default App;
