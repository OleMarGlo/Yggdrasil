import { getPosts } from '@/utils/api'

export default function Home() {
    const posts = getPosts(10, 0)
    console.log(posts)

    return(
        <main className='flex min-h-screen'>

        </main>
    )
}
