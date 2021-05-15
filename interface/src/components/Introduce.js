const Introduce = ({ stateHistory }) => {
  return (
    <>
      <div className = 'light'>
        <h2>IOTA Streams Channels</h2>
        <p>
          IOTA Streams is a framework to define and implement secure messaging protocols.
          IOTA Streams Channels is an application of this framework which provides
          functionality to send and receive public or masked data.
        </p>
      </div>
      <div className = 'grey'>
        <div className = 'card light left'>
          <h3>Getting started</h3>
          <p>
            IOTA Streams is transport agnostic by design. In this example we will use the Tangle
            as transport, so first we will connect to a Node to be able to interact with the Tangle.
            Then we have a choice to either create a Channel as an Author or participate in an
            existing Channel.
          </p>
          <button
            className = 'green'
            onClick = {() => stateHistory.push('/connect')}
          >
            Get started
          </button>
        </div>
      </div>
    </>
  )
}


export default Introduce