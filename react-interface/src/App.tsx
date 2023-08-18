import { ReactNode } from "react"
import FileList from "./components/file-list"
import DeviceList from "./components/device-list"
import FileReceiver from "./components/file-receiver"
import ExtendableContainer from "./components/custom-components/extendable-container"
import AutoExtendableContainer from "./components/custom-components/auto-extendable-container"

import * as http from "./services/http-requests"

const App = () => {
  const devices = [
    {
      name: "test 01"
    },
    {
      name: "test 02"
    }
  ]

  const filesReceived = []

  const fileReceiverHeader: ReactNode = (
    <h2>Files received</h2>
  )

  const deviceListsHeader: ReactNode = (
    <h2>Devices</h2>
  )

  const fileListsHeader: ReactNode = (
    <h2>Files</h2>
  )

  console.log("HTTP test", http.request("127.0.0.1:7878/ping"))

  return (
    <>
      <h1>Transfer file application</h1>

      <AutoExtendableContainer header={ fileReceiverHeader } manualSwitch={ false }>
        <FileReceiver fileReceived={ filesReceived } />
      </AutoExtendableContainer>

      <ExtendableContainer header={ deviceListsHeader }>
        <DeviceList devices={ devices } />
      </ExtendableContainer>

      <ExtendableContainer header={ fileListsHeader }>
        <FileList devices={ devices } files={ [] } />
      </ExtendableContainer>
    </>
  )
}

export default App
