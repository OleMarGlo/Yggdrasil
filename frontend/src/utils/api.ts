import config from '@/config'

const baseUrl = config.url.API_URL

type GetWrapperProps = {
    path: string
    options?: object
}

export async function getPosts(
    limit: number | number = 10,
    offset: number | number = 0,
): Promise<GetPostsProps | string>{
    const queryPars = new URLSearchParams({
        limit: limit.toString(),
        offset: offset.toString(),
    })

    const path = `${baseUrl+config.yggdrasilApi.POSTS_PATH}?${queryPars}`
    return await getWrapper({ path })
}

async function getWrapper({ path, options = {} }: GetWrapperProps) {
    const baseHeaders = {
        'Content-Type': 'application/json',
    }

    const defaultOptions = { method: 'GET', headers: baseHeaders }
    const finalOptions = { ...defaultOptions, ...options }

    try {
        const response = await fetch(path, finalOptions)
        if (!response.ok) {
            throw new Error(await response.text())
        }
        const data = await response.json()
        return data
        // esling-disable-next-line
    } catch (error: any) {
        console.log(error)
        return (
            JSON.stringify(error.error) ||
            JSON.stringify(error.message) ||
            'unknown error'
        )
    }
}