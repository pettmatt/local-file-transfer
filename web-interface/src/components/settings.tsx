import { useEffect, useState } from "react"
import { LocalStoragePlaceholders, SetterFunction } from "../interfaces/settings"

const Settings = () => {
    const [name, setName] = useState("")
    const [address, setAddress] = useState("")
    const [port, setPort] = useState("")

    useEffect(() => {
        checkIfLocalStorageIsEmpty(localStorages, placeholderValues)
    }, [])

    const handleChange = (event) => {
        const target = event.target
        const value = target.value
        localStorage.setItem(target.name, value)
    }

    const localStorages = ["name", "address", "port"]
    const placeholderValues: LocalStoragePlaceholders = {
        name: {
            value: "",
            setter: (value) => setName(value)
        },
        address: {
            value: "http://127.0.0.1",
            setter: (value) => setAddress(value)
        },
        port: {
            value: "7878",
            setter: (value) => setPort(value)
        }
    }

    return (
        <div>
            <label htmlFor="name">Name</label>
            <input type="text" name="name" onChange={ handleChange } />
            <br/>
            <label htmlFor="address">Address</label>
            <input type="text" name="address" onChange={ handleChange } />
            <label htmlFor="port">Port</label>
            <input type="number" name="port" onChange={ handleChange } />
            <p>Name: { name }</p>
            <p>Server address: { address }{ (port) && ":" }{ port }</p>
        </div>
    )
}

export default Settings

const checkIfLocalStorageIsEmpty = (localStorages: string[], placeholders: LocalStoragePlaceholders) => {
    localStorages.forEach(storageName => {
        const value = localStorage.getItem(storageName)
        const placeholder = placeholders[storageName as keyof LocalStoragePlaceholders]
        const setter = (placeholder as { setter: SetterFunction }).setter

        if (value === null) {
            localStorage.setItem(storageName, placeholder.value)
            setter(placeholder.value)
        }

        else {
            setter(value)
        }
    })
}