import { getPosts } from '@/utils/api'
import en from '@/text/postsPage/en.json'
import PostListItem from '@/components/post/post'

export default async function Page() {
    const { headline, subheadline, about, mission, cta } = en.posts

    const posts = await getPosts()
    if (typeof posts === 'string') {
        return (
            <div>
                <h1>No posts are able to be fetched</h1>
            </div>
        )
    }
    return (
        <div className='h-full grid relative grid-rows-[auto,1fr] place-items-center p-2'>
            <div className='grid gap-3 text-center px-20'>
                <div>
                    <h1 className='text-[clamp(1.5rem,5vw,3rem)] font-bold mt-0 gradient-bg-headers'>{headline}</h1>
                    <h2 className='text-xl text-yggdrasil-300 mt-1'>{subheadline}</h2>
                </div>
                <p className='mt-2'>{about}</p>
                <p className='mt-1'>{mission}</p>
                <p className='mt-3 font-semibold'>{cta}</p>
            </div>
            <ul className='grid grid-cols-1 sm:grid-cols-2 max-w-3xl relative h-full text-center p-2 w-full'>
                {posts.posts.map((post, index) => (
                    <li
                        key={index}
                        className='p-3 flex flex-col justify-between'
                    >
                        <PostListItem
                            key={post.id}
                            post={post}
                        />
                    </li>
                ))}
            </ul>
        </div>
    )
}