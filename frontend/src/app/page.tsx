import en from '@/text/welcomePage/en.json'

export default function Home() {
    const { headline, subheadline, about, mission, cta } = en.intro

    return(
        <div className='h-full grid relative place-items-center p-2'>
            <div className='grid gap-3 text-center px-30'>
                <div>
                    <h1 className='text-[clamp(1.5rem,5vw,3rem)] font-bold mt-0 gradient-bg-headers'>{headline}</h1>
                    <h2 className='text-xl text-yggdrasil-300 mt-1'>{subheadline}</h2>
                </div>
                <p className='mt-1'>{mission}</p>
                <p className='mt-2'>{about}</p>
                <p className='mt-3 font-semibold'>{cta}</p>
            </div>
        </div>
    )
}
