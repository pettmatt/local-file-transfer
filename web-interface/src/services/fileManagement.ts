export const downloadFile = async (stream: ReadableStream | null, filename: string, contentLength: number, setLoading: React.Dispatch<React.SetStateAction<any>>) => {
    try {
        const rs: ReadableStream = stream as ReadableStream
        const blob: Blob = await streamToBlob(rs, contentLength, setLoading)

        const blobURL = URL.createObjectURL(blob)

        const downloadLink = document.createElement("a")
        downloadLink.href = blobURL
        downloadLink.download = filename as string
        downloadLink.style.display = "none"

        console.log("Starting download process")
        document.body.appendChild(downloadLink)
        downloadLink.click()

        document.body.removeChild(downloadLink)
        URL.revokeObjectURL(blobURL)
    }
    catch (error) {
        console.log("Error occured while processing stream", error)
        setLoading(null)
    }
}

const streamToBlob = async (stream: ReadableStream, contentLength: number, setLoading: React.Dispatch<React.SetStateAction<any>>) => {
    const reader = stream.getReader()
    const chunks = []
    
    let downloadSize = 0

    console.log("Processing stream to a blob")
    const condition: boolean = true

    while (condition) {
        const { value, done } = await reader.read()
        if (done) break

        chunks.push(value)
        downloadSize += value.length
        setLoading(Math.floor((downloadSize / contentLength) * 100))
    }

    console.log("Blob is ready")
    setLoading(null)

    return new Blob(chunks)
}