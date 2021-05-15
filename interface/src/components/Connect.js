import { useState } from 'react'

const Connect = ({ stateHistory }) => {
  const [stateNodeUrl, setNodeUrl] = useState('');

  const connect = (url) => {
    let fetchBody = {
      url: url
    }
    let fetchUrl = "connect"
    let fetchOptions = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(fetchBody)
    }

    fetch(fetchUrl, fetchOptions).then(stateHistory.push('/participate'));
  }

  const connectCustom = (event) => {
    event.preventDefault();
    connect(stateNodeUrl);
  }

  return (
    <>
      <div className = 'light'>
        <h2>Connect to a Node</h2>
        <p>
          We can choose to connect to default public Nodes interacting with the Testnet or
          Mainnet or we can provide a URL to a Node of our choice.
        </p>
      </div>
      <div className = 'grey'>
        <div className = 'card light center'>
          <h3>Default nodes</h3>
          <button
            className = 'green'
            onClick = {() => connect('https://api.lb-0.testnet.chrysalis2.com')}
          >
            Testnet
          </button>
          <button
            className = 'green'
            onClick = {() => connect('https://chrysalis-nodes.iota.org')}
          >
            Mainnet
          </button>
        </div>
        <div className = 'card light center'>
          <h3>Custom node</h3>
          <form onSubmit = {connectCustom}>
            <input
              className = 'light fit'
              type = 'text'
              placeholder = 'Node URL...'
              text = {stateNodeUrl}
              onChange = {(event) => setNodeUrl(event.target.value)}
              autoFocus
            />
            <input className = 'green fit' type = 'submit' value = 'Connect' />
          </form>
        </div>
      </div>
    </>
  )
}


export default Connect