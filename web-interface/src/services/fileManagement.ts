export const downloadFile = async (stream: ReadableStream | null, filename: string | object) => {
    try {
        const rs: ReadableStream = stream as ReadableStream
        const blob: Blob = await streamToBlob(rs)

        const blobURL = URL.createObjectURL(blob)

        const downloadLink = document.createElement("a")
        downloadLink.href = blobURL
        downloadLink.download = filename as string
        downloadLink.style.display = "none"

        console.log("Starting download step")
        document.body.appendChild(downloadLink)
        downloadLink.click()

        document.body.removeChild(downloadLink)
        URL.revokeObjectURL(blobURL)
    }
    catch (error) {
        console.log("Error occured while processing stream", error)
    }
}

const streamToBlob = async (stream: ReadableStream) => {
    const reader = stream.getReader()
    const chunks = []

    console.log("Processing stream to blob")
    const condition: boolean = true

    while (condition) {
        const { value, done } = await reader.read()
        if (done) break

        chunks.push(value)
    }

    console.log("Blob is ready")

    return new Blob(chunks)
}