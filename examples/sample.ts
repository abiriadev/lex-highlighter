import { useState, ReactElement } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App(): ReactElement {
  const [count, setCount] = useState(0)
  const sitename: string = "vitejs.dev"

  return <>
    <a href={`https://${sitename}`} target="_blank" >
      <img src={viteLogo} className="logo" alt="Vite logo" />
    </a>
    <h1>Vite + React</h1>
    <div className="card">
      <button onClick={async () =>
        setCount(count => count + 1)
      }>count is {count}</button>
      <p>Edit <code>src/App.tsx</code> and save to test HMR</p>
    </div>
  </>
}

export default App
