import { ReactNode } from "react"

export interface fileListProps {
    devices: object
}

type customFunction = (parameter: string) => void | object

export interface uploadListProps {
    fileList: Array<object>,
    setFiles: React.Dispatch<React.SetStateAction<never[]>> | React.Dispatch<React.SetStateAction<File[]>>,
    removeFile: customFunction,
    downloadFile?: customFunction
}

export interface fileReceiverProps {
    fileReceived: File[]
}

export interface deviceListProps {
    devices: Array<object>
}

export interface customDeviceListProps {
    array: Array<object>
}

export interface Device {
    name: string
}

export interface extendableContainerProps {
    header: ReactNode,
    children: ReactNode,
    manualSwitch?: boolean
}