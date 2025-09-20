import Link from 'next/link'
import Logo from '../logo/logo'
import ThemeSwitch from '../theme/themeSwitch'

export default function NavBar() {
    return (
        <nav className='bg-yggdrasil-700 fixed w-full'>
            <div className='grid grid-cols-3 justify-between items-center p-4 px-20 fill-yggdrasil-50'>
                <Logo className='fill-yggdrasil-50 size-14'/>
                <div className='grid gap-8 grid-flow-col justify-center divide-x divide-yggdrasil-200 border'>
                    <Link href='/'>Posts</Link>
                    <Link href='/'>Skills</Link>
                </div>
                <div className='justify-self-end'>
                    <ThemeSwitch />
                </div>
            </div>
        </nav>
    )
}

