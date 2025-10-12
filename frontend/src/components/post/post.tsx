import SpotlightLink from '../hover/SpitlightLink'

type PostListProp = {
    post: Post
}

export default async function PostListItem({ post }: PostListProp) {
    console.log(post)
    return (
        <SpotlightLink href={`/posts/${post.slug}`}>
            <span>{typeof post === 'string' ? post : post.title}</span>
            <div className='flex justify-between text-sm mt-2'>
                <p>{post.category}</p>
                <p className='hidden sm:block'>
                    {new Date(post.updated_at).toLocaleDateString()}
                </p>
            </div>
        </SpotlightLink>
    )
}