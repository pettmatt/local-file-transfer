export const getServerAddress = (path: string = "") =>
    `${ localStorage.getItem("address") }:${ localStorage.getItem("port") }${ path }`