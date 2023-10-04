import { useEffect, useState } from "react"
import { LocalStoragePlaceholders, SetterFunction } from "../interfaces/settings"
import SettingsIcon from "@mui/icons-material/Settings"

const Settings = () => {
    const [showSettings, setShowSettings] = useState(false)
    const [address, setAddress] = useState("")
    const [name, setName] = useState("")
    const [port, setPort] = useState("")

    useEffect(() => {
        checkIfLocalStorageIsEmpty(localStorages, placeholderValues)
    }, [])

    const handleChange = (event) => {
        const target = event.target
        const value = target.value
        localStorage.setItem(target.name, value)
        placeholderValues[target.name].setter(value)
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
        <div className="settings-container">
            <button onClick={ () => setShowSettings(!showSettings) }>
                <SettingsIcon />
            </button>
            <div className={`settings overlay${ (!showSettings) ? " hidden" : "" }`}>
                <div>
                    <label htmlFor="name">Name</label>
                    <input type="text" name="name" onChange={ handleChange } />
                </div>
                <div>
                    <label htmlFor="address">Address</label>
                    <input type="text" name="address" onChange={ handleChange } />
                </div>
                <div>
                    <label htmlFor="port">Port</label>
                    <input type="number" name="port" onChange={ handleChange } />
                </div>
                <div className="settings-details">
                    <p><span>Name:</span> <i>{ name }</i></p>
                    <p><span>Server address:</span> <i><a>{ address }{ (port) && ":" }{ port }</a></i></p>
                </div>
                <div>
                    <button className="minimal-width" onClick={ () => setShowSettings(!showSettings) }>Close</button>
                </div>
            </div>
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