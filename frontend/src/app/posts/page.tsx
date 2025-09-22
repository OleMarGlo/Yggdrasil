import { getPosts } from '@/utils/api'

export default async function Page() {
    const posts = await getPosts()
    if (typeof posts === 'string') {
        return (
            <div>
                <h1>No posts are able to be fetched</h1>
            </div>
        )
    }

    console.log(posts.posts)

    return (
        <div className=''>
            <ul>
                {posts.posts.map((post, index) => (
                    <li key={index}>
                        {typeof post === 'string' ? post : post.title}
                    </li>
                ))}
            </ul>
        </div>
    )
}