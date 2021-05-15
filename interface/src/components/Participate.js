import { useState } from 'react'


const Participate = ({ stateHistory }) => {
  const [stateSeed, setSeed] = useState('')

  const participate = (multiBranching) => {
    let fetchUrl = "participate"
    let fetchOptions = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(
        {
          seed: stateSeed,
          multiBranching: multiBranching
        }
      )
    }

    fetch(fetchUrl, fetchOptions).then(stateHistory.push('/interact'))
  }

  return (
    <>
      <div className = 'light'>
        <h3>Set up a Channel</h3>
        <p>
          To send and receive messages we need to have a Channel ready. We can either create one or
          start listening to an existing one. When creating a Channel we become the Author of that
          Channel and can choose between single branch or multibranch.
        </p>
      </div>
      <div className = 'grey'>
        <div className = 'card light center'>
          <input
            className = 'light fit'
            type = 'text'
            placeholder = 'Random seed...'
            text = {stateSeed}
            onChange = {(event) => setSeed(event.target.value)}
            autoFocus
          />
          <hr/>
          <button
            className = 'green fit'
            onClick = {() => participate(false)}
          >
            Create a single branch channel
          </button>
          <button
            className = 'green fit'
            onClick = {() => participate(true)}
          >
            Create a multi branch channel
          </button>
          <hr/>
          <button
            className = 'green fit'
            onClick = {() => participate(undefined)}
          >
            Participate in a channel
          </button>
        </div>
      </div>
    </>
  )
}


export default Participate