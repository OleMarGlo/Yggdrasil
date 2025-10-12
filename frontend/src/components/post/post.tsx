import SpotlightLink from '../hover/SpitlightLink'

type PostListProp = {
    post: Post
}

export default async function PostListItem({ post }: PostListProp) {
    return (
        <SpotlightLink href={`/posts/${post.slug}`}>
            <div className='flex flex-col items-center justify-center text-sm w-full h-full'>
                <h1 className='truncate w-full py-2 text-[1.2rem]'>{post.title}</h1>
                <div className='grid grid-cols-1 xs:grid-cols-2 w-full py-2'>
                    <p className='truncate w-full'>{post.category}</p>
                    <p>{new Date(post.updated_at).toLocaleDateString()}</p>
                </div>
            </div>
        </SpotlightLink>
    )
}