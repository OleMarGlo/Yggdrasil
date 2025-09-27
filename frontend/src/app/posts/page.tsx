import { getPosts } from '@/utils/api'
import en from '@/text/postsPage/en.json'
import Welcome from '@/components/welcome/welcome'

export default async function Page() {
    const posts = await getPosts()
    if (typeof posts === 'string') {
        return (
            <div>
                <h1>No posts are able to be fetched</h1>
            </div>
        )
    }

    return (
        <div className='w-full h-full'>
            <Welcome />
            <ul className='grid-cols-3'>
                {posts.posts.map((post, index) => (
                    <li key={index}>
                        {typeof post === 'string' ? post : post.title}
                    </li>
                ))}
            </ul>
        </div>
    )
}