
import Link from 'next/link'
import Image from 'next/image'

export default function NotFound() {
    return (
        <div className='fixed inset-0 flex flex-col items-center justify-center bg-yggdrasil-950 text-center'>
            <h1 className='text-5xl font-bold mb-4'>You never saw this page</h1>
            <Image src='/elmoGun.gif' alt='' width={600} height={100} className='block' />
            <Link href='/' className='mt-6 underline'>
                Go back IMMEDIATELY
            </Link>
        </div>
    )
}
