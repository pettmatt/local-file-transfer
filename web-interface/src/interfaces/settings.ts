export type SetterFunction = (parameter: string) => void

export interface LocalStoragePlaceholders {
    name: {
        value: string,
        setter: SetterFunction
    },
    address: {
        value: string,
        setter: SetterFunction
    },
    port: {
        value: string,
        setter: SetterFunction
    }
}