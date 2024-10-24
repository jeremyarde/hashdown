import { useState } from "react";
import "./App.css";
import { Button } from "./components/ui/button";
import Layout from "./app/layout";

function App() {
  const [count, setCount] = useState(0);

  return (
    <>
      <Layout>
        <Button>Click me</Button>
      </Layout>
    </>
  );
}

export default App;
