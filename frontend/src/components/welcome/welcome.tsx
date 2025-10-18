import en from '@/text/postsPage/en.json'

export default function Welcome() {
    const { headline, subheadline, about, mission, cta } = en.intro

    return (
        <div className='grid text-center'>
            <div className=''>
                <h1 className='text-4xl font-bold'>{headline}</h1>
                <h2 className='text-xl text-gray-600'>{subheadline}</h2>
            </div>
            <p className='mt-4'>{about}</p>
            <p className='mt-2'>{mission}</p>
            <p className='mt-6 font-semibold'>{cta}</p>
        </div>
    )
}