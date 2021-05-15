import { Switch, Route } from 'react-router-dom'
import { useHistory } from 'react-router-dom'

import Introduce from './components/Introduce'
import Connect from './components/Connect'
import Participate from './components/Participate'
import Stream from './components/Stream'
import Logo from './assets/logo.svg'

const App = () => {
  const stateHistory = useHistory()

  return (
    <>
      <main>
        <Switch>
          <Route path = '/participate'>
            <Participate stateHistory = {stateHistory}/>
          </Route>
          <Route path='/connect'>
            <Connect stateHistory = {stateHistory}/>
          </Route>
          <Route path='/stream'>
            <Stream/>
          </Route>
          <Route path='/'>
            <Introduce stateHistory = {stateHistory}/>
          </Route>
        </Switch>
      </main>
      <footer className = 'dark center'>
        <img src = {Logo} style = {{height: '4rem'}} alt = "IOTA"/>
      </footer>
    </>
  )
}


export default App