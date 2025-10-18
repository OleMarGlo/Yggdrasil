export default async function Page({ params,}: { params: Promise<{ slug: string}>}) {
    const { slug } = await params
    return (
        <div className='text-yggdrasil text-9xl'>
            My Post: {slug}
        </div>
    )
}