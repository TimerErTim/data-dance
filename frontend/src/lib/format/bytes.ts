export function formatBytesBase2(bytes: number, decimals: number = 1) {
    const units = ['bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];

    let l = 0, n = bytes;

    while (n >= 1024 && ++l) {
        n = n / 1024;
    }

    return (n.toFixed(l > 0 ? (n >= 10 ? decimals : decimals + 1) : 0) + ' ' + units[l]);
}

export function formatBytesBase10(bytes: number, decimals: number = 2) {
    const units = ['bytes', 'kB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];

    let l = 0, n = bytes;

    while (n >= 1000 && ++l) {
        n = n / 1000;
    }

    return (n.toFixed(l > 0 ? (n >= 10 ? decimals : decimals + 1) : 0) + ' ' + units[l]);
}
