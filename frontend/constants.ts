import packageInfo from './package.json'

const isServer = typeof window === 'undefined'
const requerdEnvVars = [
    'API_URL',
]

const missingEnvVars = requerdEnvVars.filter(envVar => !process.env[envVar])

if (isServer && missingEnvVars.length > 0) {
    throw new Error(
        `Missing environment variables: ${missingEnvVars.join(', ')}`
    )
}

const env = Object.fromEntries(
    requerdEnvVars.map((key) => [key, process.env[key]])
)


const config = {
    url: {
        API_URL: env.API_URL,
    },
    yggdrasilApi: {
        POSTS_PATH: '/posts',
        CATEGORIES_PATH: '/categories',
    },
    version: packageInfo.version,
}

export default config