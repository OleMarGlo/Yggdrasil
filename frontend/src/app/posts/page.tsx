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
        <div className='w-full min-h-screen grid place-items-center px-[clamp(1.5rem,5vw,3rem)]'>
            <div className='grid gap-4 text-center'>
                <div>
                    <h1 className='text-[clamp(1.5rem,5vw,3rem)] font-bold'>{headline}</h1>
                    <h2 className='text-xl text-yggdrasil-300'>{subheadline}</h2>
                </div>
                <p className='mt-4'>{about}</p>
                <p className='mt-2'>{mission}</p>
                <p className='mt-6 font-semibold'>{cta}</p>
            </div>

            <ul className='grid grid-cols-2 gap-4 max-w-3xl text-center w-full'>
                {posts.posts.map((post, index) => (
                    <li
                        key={index}
                        className='border p-3 flex flex-col justify-between'
                    >
                        <span>{typeof post === 'string' ? post : post.title}</span>
                        <div className='flex justify-between text-sm mt-2'>
                            <p>{post.category}</p>
                            <p className='hidden sm:block'>
                                {new Date(post.updated_at).toLocaleDateString()}
                            </p>
                        </div>
                    </li>
                ))}
            </ul>
        </div>
    )
}