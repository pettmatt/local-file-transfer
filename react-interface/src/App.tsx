import { useEffect, useState } from "react"
import FileList from "./components/file-list"
import DeviceList from "./components/device-list"
import FileReceiver from "./components/file-receiver"
import ExtendableContainer from "./components/custom-components/extendable-container"
import AutoExtendableContainer from "./components/custom-components/auto-extendable-container"

import * as http from "./services/http-requests"

const App = () => {
  const [devices, setDevices] = useState([])
  const [filesReceived, setFileReceived] = useState([])

  // const devices = [
  //   {
  //     name: "test 01"
  //   },
  //   {
  //     name: "test 02"
  //   }
  // ]

  useEffect(() => {
    http.request("http://127.0.0.1:7878/ping")
      .then(() => {
        console.log("Local backend is on")
        http.request("http://127.0.0.1:7878/devices")
          .then(response => setDevices(response))
          .catch(error => console.warn("Devices fetch error:", error))
      })
      .catch(error => console.warn("Seems like local backend is not on:", error))

    console.log("Devices:", devices)
  }, [])


  return (
    <>
      <h1>Transfer file application</h1>

      <AutoExtendableContainer header={ <h2>Files received</h2> } manualSwitch={ false }>
        <FileReceiver fileReceived={ filesReceived } />
      </AutoExtendableContainer>

      <ExtendableContainer header={ <h2>Devices</h2> }>
        <DeviceList devices={ devices } />
      </ExtendableContainer>

      <ExtendableContainer header={ <h2>Files</h2> }>
        <FileList devices={ devices } />
      </ExtendableContainer>
    </>
  )
}

export default App
