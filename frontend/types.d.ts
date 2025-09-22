type GetPostsProps = {
    count: number
    posts: {
        id: number,
        title: string,
        slug: string,
        content: string,
        created_at: string,
        updated_at: string,
        category: string,
    }[]
}

type GetPostProps = {
    id: number
    title: string
    slug: string
    content: string
    created_at: string
    updated_at: string
    category: string
}

type GetCategoriesProps = {
    count: number
    categories: {
        id: number,
        category: string,
        slug: string,
        description: string,
    }[]
}

type GetCategoryProps = {
    id: number
    category: string
    slug: string
    description: string
}
