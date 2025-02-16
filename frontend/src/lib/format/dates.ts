export function formatDate(date: Date) {
    const format = Intl.DateTimeFormat('de-DE', {dateStyle: 'medium'})
    return format.format(date)
}

export function formatTime(date: Date) {
    const format = Intl.DateTimeFormat('de-DE', {timeStyle: 'short'})
    return format.format(date)
}

export function formatTimeWithSeconds(date: Date) {
    const format = Intl.DateTimeFormat('de-DE', {timeStyle: 'medium'})
    return format.format(date)
}

export function formatDateTime(date: Date) {
    const format = Intl.DateTimeFormat('de-DE', {dateStyle: 'medium', timeStyle: 'medium'})
    return format.format(date)
}
