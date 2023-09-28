import Settings from "./components/settings"
import FileList from "./components/file-list"
import FileSender from "./components/file-sender"
import ExtendableContainer from "./components/custom-components/extendable-container"

const App = () => {

  return (
    <>
      <h1>Local file transfer</h1>

      <Settings />

      <FileList />
      <FileSender />
    </>
  )
}

export default App
