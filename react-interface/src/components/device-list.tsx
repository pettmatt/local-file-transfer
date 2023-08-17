import { useState } from "react"

interface DeviceListProps {
    devices: Array<object>
}

const DeviceList = (props: DeviceListProps) => {
    const [devices, setDevices] = useState(props.devices)

    const devicesListed = devices.map(device => (
        <DeviceItem key={ device.id } device={ device } />
    ))

    return (
        <div className="section-container">
            { devices.length > 0 ? (
                <div className="list-container">
                    { devicesListed }
                </div>
            )
            :
            (
                <div className="notification-container no-devices">
                    <h2>No devices found</h2>
                    <p>
                        <small>
                            Please make sure you're connected to the local network through WiFi or Ethernet cable.
                        </small>
                    </p>
                </div>
            ) }
        </div>
    )
}

interface DeviceItemProps {
    device: object
}

const DeviceItem = (props: DeviceItemProps) => {
    const [device, setDevice] = useState(props.device)

    const handleVerify = () => { console.log("Clicked") }

    return (
        <button key={ device.id } onClick={ handleVerify }>
            { device.localIp }
        </button>
    )
}

export default DeviceList
