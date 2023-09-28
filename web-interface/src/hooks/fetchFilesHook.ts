import { useState } from "react"

const useFetchFilesHook = () => {
    const [fetchFiles, setFetch] = useState(true)

    const activate = () => {
        setFetch(true)
    }

    const deactivate = () => {
        setFetch(false)
    }

    return { fetchFiles, activate, deactivate }
}

export default useFetchFilesHook