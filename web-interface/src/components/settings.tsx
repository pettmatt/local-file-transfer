import { useEffect, useState, ChangeEvent, useMemo } from "react"
import { LocalStoragePlaceholders, SetterFunction } from "../interfaces/settings"
import SettingsIcon from "@mui/icons-material/Settings"

const Settings = () => {
    const [showSettings, setShowSettings] = useState(false)
    const [address, setAddress] = useState("")
    const [name, setName] = useState("")
    const [port, setPort] = useState("")

    const localStorages = useMemo(() => {
        return ["name", "address", "port"]
    }, [])

    const placeholderValues: LocalStoragePlaceholders = useMemo(() => {
        return {
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
    }, [setName, setAddress, setPort])

    const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
        const name = event.target.name
        const value = event.target.value
        localStorage.setItem(name, value)
        // eslint-disable-next-line @typescript-eslint/ban-ts-comment
        // @ts-ignore
        placeholderValues[name].setter(value)
    }

    useEffect(() => {
        checkIfLocalStorageIsEmpty(localStorages, placeholderValues)
    }, [localStorages, placeholderValues])

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