import { useEffect } from "react"
import Settings from "./components/settings"
import FileList from "./components/file-list"
import FileSender from "./components/file-sender"
import useFetchFilesHook from "./hooks/fetchFilesHook"

const App = () => {
  const { fetchFiles, activate, deactivate } = useFetchFilesHook()

  useEffect(() => {
    // Check which theme setting the browser uses
    const prefersDarkMode = window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches

    if (prefersDarkMode) {
      // Import the dark mode CSS file dynamically
      import("./styles/dark-mode.css")
        .then(() => console.log("Dark mode styling imported"))
        .catch(error => console.error("Error importing dark mode CSS:", error))
    }
  }, [])


  return (
    <>
      <div className="header">
        <h1>Local file transfer</h1>
        <Settings />
      </div>

      <FileList fileHook={ { fetchFiles, activate, deactivate } } />
      <FileSender fileHook={ { fetchFiles, activate, deactivate } } />
    </>
  )
}

export default App
