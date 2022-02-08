export const API_URL = "http://localhost:8080"

const baseFetchOptions: RequestInit = {
    mode: 'cors',
    credentials: 'include',
    headers: {
        'Content-Type': 'application/json'
    },
}

export async function postJson(address: string, body: unknown) {
    const url = new URL(address, API_URL).toString()

    const res = await fetch(url, {
        ...baseFetchOptions,
        method: 'POST',
        body: JSON.stringify(body)
    })
    return res.json()
}