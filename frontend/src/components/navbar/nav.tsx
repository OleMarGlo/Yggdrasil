import Link from 'next/link'
import Logo from '../logo/logo'
import ThemeSwitch from '../theme/themeSwitch'
import SpotlightGroup from '../hover/hoverFollow'

export default function NavBar() {
    return (
        <nav className='bg-yggdrasil-700/50 fixed w-full backdrop-blur-md z-50'>
            <div className='mx-auto grid grid-cols-3 items-center px-20 h-full p-2'>
                <div className='flex justify-start'>
                    <Link href='/'>
                        <Logo className='fill-foreground size-14 transition-colors duration-300' />
                    </Link>
                </div>
                <div className='flex justify-center'>
                    <div className='hidden md:block'>
                        <SpotlightGroup />
                    </div>
                </div>
                <div className='flex justify-end'>
                    <ThemeSwitch />
                </div>
            </div>
        </nav>
    )
}


