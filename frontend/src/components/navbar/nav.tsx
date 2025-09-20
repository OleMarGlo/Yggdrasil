import Link from 'next/link'
import Logo from '../logo/logo'
import ThemeSwitch from '../theme/themeSwitch'
import SpotlightGroup from '../hover/hoverFollow'

export default function NavBar() {
    return (
        <nav className='bg-yggdrasil-700/50 fixed w-full'>
            <div className='grid grid-cols-3 justify-between items-center px-40 fill-yggdrasil-50'>
                <div className='justify-self-start'>
                    <Link href='/'>
                        <Logo className='fill-foreground size-14 transition-colors duration-300'/>
                    </Link>
                </div>
                <SpotlightGroup/>
                <div className='justify-self-end h-auto hover:bg-yggdrasil-600'>
                    <ThemeSwitch  />
                </div>
            </div>
        </nav>
    )
}

