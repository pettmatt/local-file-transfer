import { useEffect, useState } from "react"
import Settings from "./components/settings"
import FileList from "./components/file-list"
// import DeviceList from "./components/device-list"
import FileReceiver from "./components/file-receiver"
import ExtendableContainer from "./components/custom-components/extendable-container"
import { getServerAddress } from "./services/localStorage"
// import AutoExtendableContainer from "./components/custom-components/auto-extendable-container"

interface Device {
  name: string
}

const App = () => {
  const [devices, setDevices] = useState<object[]>([])
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const [filesReceived, setFileReceived] = useState([])

  useEffect(() => {
    fetchDevices()
  }, [])

  const fetchDevices = () => {
    // The IP should be dynamic
    fetch(getServerAddress("/devices"))
      .then(response => {
        if (response.ok) return response.json()

        throw new Error("Response was not OK")
      })
      .then(data => {
        const responseDevices: Array<object> = data.devices

        if (devices.length === responseDevices.length) {

          const newList = new Array(responseDevices.length)
          newList.push(responseDevices)

          Array.from(responseDevices).filter(device => {
            Array.from(devices).map(oldDevice => {
              const d = device as Device
              const od = oldDevice as Device

              if (d.name !== od.name)
                newList.push(d)

            })
          })

          setDevices([...newList])
        }

        else setDevices(responseDevices)
      })
      .catch(error => console.warn("Device fetch error:", error))
  }

  return (
    <>
      <h1>Local file transfer</h1>

      <Settings />

      <ExtendableContainer header={ <h2>Files received</h2> } manualSwitch={ false }>
        <FileReceiver fileReceived={ filesReceived } />
      </ExtendableContainer>

      {/* <ExtendableContainer header={
        <>
          <h2>Shared</h2>
          <button onClick={ fetchDevices }>Update</button>
        </>
      }>
        <DeviceList devices={ devices } />
      </ExtendableContainer> */}

      <ExtendableContainer header={ <h2>Personal</h2> }>
        <FileList />
      </ExtendableContainer>
    </>
  )
}

export default App
