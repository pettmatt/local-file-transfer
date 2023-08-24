interface RequestInit {
    method: string,
    body?: string
}

export const request = (address: string = "http://127.0.0.1:7878", method: string = "GET", body: object = {}) => {
    const init: RequestInit = {
        method: method,
    }

    if (method === "POST" || method === "PUT" || method === "DELETE") {
        init.body = JSON.stringify(body)
    }

    const request: Request = new Request(address, init)

    const response = fetch(request)
        .then(response => {
            if (!response.ok) {
                throw new Error(`HTTP request response was not OK: ${ response }`)
            }

            return response.json()
        })
        .catch(error => {
            // console.warn("Error occured while making HTTP request:", error)
            throw new Error(`Error occured while making HTTP request: \n${ error }`)
        })

    return response
}