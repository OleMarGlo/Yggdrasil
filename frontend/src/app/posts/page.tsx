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
        <div className='w-full h-full grid relative grid-rows-[auto,1fr] place-items-center p-2'>
            <div className='grid gap-3 text-center mt-0'>
                <div>
                    <h1 className='text-[clamp(1.5rem,5vw,3rem)] font-bold mt-0'>{headline}</h1>
                    <h2 className='text-xl text-yggdrasil-300 mt-1'>{subheadline}</h2>
                </div>
                <p className='mt-2'>{about}</p>
                <p className='mt-1'>{mission}</p>
                <p className='mt-3 font-semibold'>{cta}</p>
            </div>

            <ul className='grid grid-cols-2 gap-4 max-w-3xl relative h-full text-center'>
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