import Settings from "./components/settings"
import FileList from "./components/file-list"
import FileSender from "./components/file-sender"
import useFetchFilesHook from "./hooks/fetchFilesHook"

const App = () => {
  const { fetchFiles, activate, deactivate } = useFetchFilesHook()

  return (
    <>
      <h1>Local file transfer</h1>

      <Settings />

      <FileList fileHook={ { fetchFiles, activate, deactivate } } />
      <FileSender fileHook={ { fetchFiles, activate, deactivate } } />
    </>
  )
}

export default App
