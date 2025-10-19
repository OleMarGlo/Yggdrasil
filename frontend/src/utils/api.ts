import config from '@/config'

const baseUrl = config.url.API_URL

type GetWrapperProps = {
    path: string
    options?: object
}

export async function getPosts(
    limit: number | number = 20,
    offset: number | number = 0,
): Promise<GetPostsProps | string>{
    const queryPars = new URLSearchParams({
        limit: limit.toString(),
        offset: offset.toString(),
    })

    const path = `${baseUrl+config.yggdrasilApi.POSTS_PATH}?${queryPars}`
    return await getWrapper({ path })
}

export async function getPost(
    id: number,
): Promise<GetPostProps | string> {
    const path = `${baseUrl+config.yggdrasilApi.POSTS_PATH}/${id}`
    return await getWrapper({ path })
}

export async function getCategories(): Promise<GetCategoriesProps | string> {
    const path = `${baseUrl+config.yggdrasilApi.CATEGORIES_PATH}`
    return await getWrapper({ path })
}

export async function getActiveCategories(): Promise<GetCategoriesProps | string> {
    const path = `${baseUrl+config.yggdrasilApi.CATEGORIES_PATH}/active`
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
        // eslint-disable-next-line
    } catch (error: any) {
        console.log(error)
        return (
            JSON.stringify(error.error) ||
            JSON.stringify(error.message) ||
            'unknown error'
        )
    }
}