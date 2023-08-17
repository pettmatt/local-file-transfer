import DeviceList from "./components/device-list"
import FileList from "./components/file-list"

const App = () => {
  const devices = [
    {
        id: 0,
        localIp: "test 01"
    },
    {
        id: 1,
        localIp: "test 02"
    }
  ];

  return (
    <>
      <h1>Transfer file application</h1>

      <DeviceList devices={ devices } />
      <FileList devices={ devices } files={ [] } />
    </>
  )
}

export default App
