import { getPosts } from '@/utils/api'
import en from '@/text/postsPage/en.json'

export default async function Page() {
    const { headline, subheadline, about, mission, cta } = en.intro

    const posts = await getPosts()
    if (typeof posts === 'string') {
        return (
            <div>
                <h1>No posts are able to be fetched</h1>
            </div>
        )
    }
    return (
        <div className='w-full mx-auto min-h-screen'>
            <div className='grid text-center'>
                <div className=''>
                    <h1 className='text-[clamp(1.5rem,5vw,3rem)] font-bold'>{headline}</h1>
                    <h2 className='text-xl text-yggdrasil-300'>{subheadline}</h2>
                </div>
                <p className='mt-4'>{about}</p>
                <p className='mt-2'>{mission}</p>
                <p className='mt-6 font-semibold'>{cta}</p>
            </div>
            <ul className='grid grid-cols-2 gap-4 max-w-3xl mx-auto text-center'>
                {posts.posts.map((post, index) => (
                    <div className='border p-3'>
                        <li key={index} >
                            {typeof post === 'string' ? post : post.title}
                        </li>
                        <div className='flex justify-between'>
                            <p >{post.category}</p>
                            <p className='hidden sm:block'>{new Date(post.updated_at).toLocaleDateString()}</p>
                        </div>
                    </div>
                ))}
            </ul>
        </div>
    )
}