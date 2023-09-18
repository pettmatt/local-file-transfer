interface RequestInit {
    method: string,
    headers?: object,
    body?: any
}

export const request = (address: string = "http://127.0.0.1:7878", method: string = "GET", req_params: object = {}) => {
    method = method.toUpperCase()
    const init: RequestInit = {
        method: method
    }

    if (method === "POST" || method === "DELETE")
        init.body = req_params.body

    const request: Request = new Request(address, init)

    const response = fetch(request)
        .then(response => {
            if (response.ok) return response.json()
            return new Error(`HTTP request response was not OK`)
        })
        .catch(error => {
            // console.warn("Error occured while making HTTP request:", error)
            return new Error(`Error occured while making HTTP request:\n${ error }`)
        })

    return response
}